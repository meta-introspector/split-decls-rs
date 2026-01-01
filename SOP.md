# Standard Operating Procedure (SOP)
## Split Declarations Genesis - Progressive Compilation System

### Overview
This SOP covers the complete workflow for using the progressive compilation system to test individual Rust declarations from the rustc codebase.

## Prerequisites
- Rust toolchain installed
- Git with submodule support
- ~5GB disk space for processed files

## Quick Start

### 1. Setup
```bash
git clone --recursive <repo-url>
cd split-decls-genesis
```

### 2. Generate Files
```bash
cargo run --bin runbuild
```

### 3. Test Compilation
```bash
cargo run --bin unified_driver > results.log 2>&1
```

### 4. Check Results
```bash
grep "✅ Success" results.log | wc -l
grep "❌ Compilation failed" results.log | wc -l
```

## Detailed Procedures

### File Generation Process

#### Command
```bash
cargo run --bin runbuild
```

#### Expected Output
- Processing messages for 3102 files
- AST trace proof generation
- Files written to `submodules/rust/` directory structure

#### Verification
```bash
find submodules/ -name "*.rs" | wc -l  # Should be ~3102
ls proofs/ | wc -l                     # Should be ~3102
```

### Progressive Testing Process

#### Command
```bash
cargo run --bin unified_driver > results.log 2>&1
```

#### Expected Behavior
- Sequential processing of all 3102 files
- Success/failure reporting for each file
- Source and binary size measurements

#### Monitoring Progress
```bash
# Watch live progress
tail -f results.log | grep -E "(Step|✅|❌)"

# Check current status
grep -E "(Step|✅|❌)" results.log | tail -10
```

### Error Analysis

#### Common Error Types
```bash
# Check error distribution
grep -E "error\[E[0-9]+\]" results.log | sort | uniq -c | sort -rn
```

#### Name Conflicts (E0255)
```bash
grep -A3 -B3 "error\[E0255\]" results.log
```
**Fix**: Rename conflicting module in `src/wrap_types.rs`

#### Missing Crates (E0433)
```bash
grep -A3 -B3 "error\[E0433\]" results.log
```
**Fix**: Add `extern crate` declaration to `src/lib.rs`

#### Unresolved Imports (E0432)
```bash
grep -A3 -B3 "error\[E0432\]" results.log
```
**Fix**: Add module stub to `src/wrap_types.rs`

### Infrastructure Updates

#### Adding External Crates
```bash
echo 'extern crate rustc_new_crate;' >> src/lib.rs
```

#### Adding Type Stubs
```bash
echo 'pub struct NewType;' >> src/wrap_types.rs
```

#### Adding Feature Flags
```bash
echo '#![feature(new_feature)]' >> src/lib.rs
```

#### Testing Changes
```bash
cargo run --bin unified_driver | head -20
```

### Performance Monitoring

#### Success Rate Tracking
```bash
# Current success rate
TOTAL=$(grep -c "Step" results.log)
SUCCESS=$(grep -c "✅ Success" results.log)
echo "Success rate: $SUCCESS/$TOTAL"
```

#### File Size Analysis
```bash
# Extract size metrics
grep "Source:" results.log | head -10
```

#### Compilation Time
```bash
time cargo run --bin unified_driver > /dev/null 2>&1
```

## Troubleshooting

### Build.rs Issues
```bash
# Check build.rs execution
cargo run --bin runbuild 2>&1 | grep -E "(ERROR|WARN)"
```

### Unified Driver Issues
```bash
# Check driver compilation
cargo build --bin unified_driver
```

### File System Issues
```bash
# Check disk space
df -h .

# Check file permissions
ls -la submodules/rust/
```

### Memory Issues
```bash
# Monitor memory usage during processing
top -p $(pgrep unified_driver)
```

## Maintenance Tasks

### Daily
- Monitor success rate trends
- Check for new compilation errors
- Review performance metrics

### Weekly
- Update rustc submodule
- Regenerate processed files
- Analyze error patterns

### Monthly
- Review and update infrastructure
- Optimize stub definitions
- Update documentation

## Quality Assurance

### Validation Checklist
- [ ] All 3102 files generated
- [ ] AST proofs created for each file
- [ ] No build.rs errors
- [ ] Unified driver compiles successfully
- [ ] At least 1 file compiles successfully

### Regression Testing
```bash
# Save current results
cp results.log results_baseline.log

# After changes, compare
diff <(grep "✅\|❌" results_baseline.log) <(grep "✅\|❌" results.log)
```

## Emergency Procedures

### Complete Reset
```bash
# Clean all generated files
rm -rf submodules/rust/
rm -rf proofs/
rm -f symbol_map.json*
rm -f src/current.rs

# Regenerate everything
cargo run --bin runbuild
```

### Partial Recovery
```bash
# If only some files are corrupted
find submodules/ -name "*.rs" -size 0 -delete
cargo run --bin runbuild  # Will regenerate missing files
```

## Success Metrics

### Target Goals
- **Success Rate**: >50% of files compile successfully
- **Processing Time**: <10 minutes for full generation
- **Error Rate**: <5% infrastructure-related errors

### Current Status
- **Files Processed**: 3102/3102 ✅
- **Success Rate**: 1/3102 (0.03%) - First successful compilation achieved
- **Infrastructure**: Fully functional ✅

## Contact Information
- **Repository**: [Link to repo]
- **Issues**: [Link to issue tracker]
- **Documentation**: README.md, this SOP
