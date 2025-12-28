#!/usr/bin/env python3
"""
Auto-dependency resolver for split-decls-rs
Scans compilation errors and generates the required module includes
"""

import re
import subprocess
import sys
from pathlib import Path

def extract_missing_dependencies(build_output):
    """Extract missing dependencies from cargo build output"""
    dependencies = {
        'modules': set(),
        'functions': set(),
        'types': set(),
        'macros': set()
    }
    
    # Parse different error patterns
    patterns = {
        'modules': r'use of unresolved module or unlinked crate `(\w+)`',
        'functions': r'cannot find function `(\w+)` in this scope',
        'types': r'cannot find type `(\w+)` in this scope',
        'macros': r'cannot find macro `(\w+)` in this scope'
    }
    
    for category, pattern in patterns.items():
        matches = re.findall(pattern, build_output)
        dependencies[category].update(matches)
    
    return dependencies

def find_declarations_in_output2(dep_name, dep_type):
    """Find declaration files in output2 for a given dependency"""
    output2_path = Path("../output2")
    
    if dep_type == 'functions':
        pattern = f"**/fn/**/{dep_name}.rs"
    elif dep_type == 'types':
        pattern = f"**/struct/**/{dep_name}.rs"
    elif dep_type == 'modules':
        pattern = f"**/{dep_name}/**/*.rs"
    else:
        return []
    
    return list(output2_path.glob(pattern))

def generate_module_includes(dependencies):
    """Generate the module include statements"""
    includes = []
    
    for dep_type, deps in dependencies.items():
        for dep in deps:
            files = find_declarations_in_output2(dep, dep_type)
            if files:
                # Take the first match for now
                file_path = files[0]
                rel_path = file_path.relative_to(Path("."))
                
                module_name = f"{dep}_{dep_type}_module"
                include_stmt = f"""
// Auto-generated module for {dep}
mod {module_name} {{
    use super::*;
    include!("{rel_path}");
}}
pub use {module_name}::{dep};
"""
                includes.append(include_stmt)
    
    return includes

def main():
    """Main auto-dependency resolution"""
    print("🔧 Running auto-dependency resolver...")
    
    # Run cargo build and capture output
    result = subprocess.run(
        ["cargo", "build"], 
        cwd=".", 
        capture_output=True, 
        text=True
    )
    
    if result.returncode == 0:
        print("✅ Build successful - no missing dependencies")
        return
    
    print("📋 Analyzing compilation errors...")
    dependencies = extract_missing_dependencies(result.stderr)
    
    print(f"Found missing dependencies:")
    for dep_type, deps in dependencies.items():
        if deps:
            print(f"  {dep_type}: {', '.join(deps)}")
    
    # Generate includes
    includes = generate_module_includes(dependencies)
    
    if includes:
        print("🔧 Generated module includes:")
        for include in includes:
            print(include)
        
        # Optionally write to a file
        with open("auto_generated_includes.rs", "w") as f:
            f.write("// Auto-generated dependency includes\n")
            f.write("\n".join(includes))
        
        print("📝 Wrote auto_generated_includes.rs")

if __name__ == "__main__":
    main()
