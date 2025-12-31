#!/bin/bash

echo "🔍 QA SYSTEM: Generator Validation"
echo "=================================="

# Function to validate a single instance
validate_instance() {
    local instance_num=$1
    local generator_name=$2
    local instance_path="instances/$instance_num"
    
    echo ""
    echo "🔍 VALIDATING INSTANCE $instance_num"
    echo "Generator: $generator_name"
    echo "Path: $instance_path"
    
    # Step 1: Check for manual changes in git history
    echo "📋 Checking git commits for manual changes..."
    cd "$instance_path" 2>/dev/null || {
        echo "❌ Instance $instance_num not found"
        return 1
    }
    
    # Look for manual fix commits
    manual_commits=$(git log --oneline | grep -i -E "(fix|manual|update|correct|patch)" | head -5)
    if [ -n "$manual_commits" ]; then
        echo "⚠️  Manual changes detected:"
        echo "$manual_commits" | sed 's/^/   /'
        echo "🔧 These changes need to be incorporated into $generator_name"
    else
        echo "✅ No manual changes detected"
    fi
    
    cd - > /dev/null
    
    # Step 2: Test generator can recreate instance
    echo "🔄 Testing generator: $generator_name"
    
    # Remove test instance if exists
    rm -rf "qa_test_$instance_num"
    
    # Run generator (it should create instances/00X, we'll rename it)
    if cargo run --bin "$generator_name" > /dev/null 2>&1; then
        echo "✅ Generator ran successfully"
        
        # Move generated instance to test location
        if [ -d "$instance_path" ]; then
            cp -r "$instance_path" "qa_test_$instance_num"
            
            # Test the generated instance
            echo "🧪 Testing generated instance..."
            cd "qa_test_$instance_num"
            
            if cargo build > /dev/null 2>&1; then
                echo "✅ Generated instance builds successfully"
                
                # Test if test_universe exists and runs
                if [ -f "src/bin/test_universe.rs" ]; then
                    if cargo run --bin test_universe > /dev/null 2>&1; then
                        echo "✅ Generated instance test runs successfully"
                    else
                        echo "❌ Generated instance test failed"
                    fi
                fi
            else
                echo "❌ Generated instance failed to build"
            fi
            
            cd - > /dev/null
            
            # Clean up
            rm -rf "qa_test_$instance_num"
        else
            echo "❌ Generator did not create expected instance"
        fi
    else
        echo "❌ Generator $generator_name failed to run"
    fi
    
    echo "✅ INSTANCE $instance_num VALIDATION COMPLETE"
}

# Validate each instance
validate_instance "002" "step2_bootstrap"
validate_instance "003" "step3_bootstrap" 
validate_instance "004" "step4_bootstrap"
validate_instance "005" "step5_bootstrap"
validate_instance "006" "step6_bootstrap"

echo ""
echo "🎉 QA VALIDATION COMPLETE"
echo "========================="
echo "All instances have been validated against their generators."
echo "Any manual changes detected should be incorporated into the generators."
