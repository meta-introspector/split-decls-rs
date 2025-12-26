#!/bin/bash

FILE="$1"
OUTPUT="$2"

if [ ! -f "$FILE" ]; then
    echo "File not found: $FILE"
    exit 1
fi

echo "🔧 Transforming $FILE -> $OUTPUT"

# Add audit macros at the top
cat bootstrap_audit_macros.rs > "$OUTPUT"
echo "" >> "$OUTPUT"

# Transform the source file
sed 's/std::process::Command/audit_execute!(std::process::Command/g' "$FILE" | \
sed 's/Command::new(/audit_execute!(Command::new(/g' | \
sed 's/\.output()/.output())/g' | \
sed 's/\.status()/.status())/g' | \
sed 's/std::fs::write(/audit_fs_write!(/g' | \
sed 's/fs::write(/audit_fs_write!(/g' | \
sed 's/std::fs::create_dir_all(/audit_fs_create_dir_all!(/g' | \
sed 's/fs::create_dir_all(/audit_fs_create_dir_all!(/g' >> "$OUTPUT"

echo "✅ Transformation complete"
