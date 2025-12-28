#!/usr/bin/env python3
"""Extract patches from parent Cargo.toml and update minimal-test Cargo.toml"""

import toml
import sys
from pathlib import Path

def main():
    # Read parent Cargo.toml
    parent_toml = Path("../../Cargo.toml")
    if not parent_toml.exists():
        print(f"❌ Parent Cargo.toml not found at {parent_toml}")
        return 1
    
    with open(parent_toml) as f:
        parent_config = toml.load(f)
    
    # Read our Cargo.toml
    our_toml = Path("Cargo.toml")
    with open(our_toml) as f:
        our_config = toml.load(f)
    
    # Extract patches from parent
    patches = parent_config.get("patch", {}).get("crates-io", {})
    
    print(f"🔍 Found {len(patches)} patches in parent Cargo.toml")
    
    # Update our patches
    if "patch" not in our_config:
        our_config["patch"] = {}
    if "crates-io" not in our_config["patch"]:
        our_config["patch"]["crates-io"] = {}
    
    # Copy all patches
    our_config["patch"]["crates-io"].update(patches)
    
    # Write updated Cargo.toml
    with open(our_toml, 'w') as f:
        toml.dump(our_config, f)
    
    print(f"✅ Updated Cargo.toml with {len(patches)} patches")
    print("📋 Patches applied:")
    for name, config in patches.items():
        if isinstance(config, dict) and "path" in config:
            print(f"  - {name} -> {config['path']}")
        else:
            print(f"  - {name} -> {config}")

if __name__ == "__main__":
    sys.exit(main())
