#!/bin/bash

# Script to rebase and remove large file from history
# Target: Remove ./level-crates/rust-ecosystem-level2-0/src/lib.rs from git history

set -e

echo "Starting rebase to remove large file from history..."

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "Current branch: $CURRENT_BRANCH"

# Target commit (one before the large file was added)
TARGET_COMMIT="5f8c48a5e245a968ea4ed6b082b167517b975e8b"

echo "Rebasing onto commit: $TARGET_COMMIT"
echo "This will remove commits that added the large file"

# Create backup branch
BACKUP_BRANCH="${CURRENT_BRANCH}_backup_$(date +%Y%m%d_%H%M%S)"
echo "Creating backup branch: $BACKUP_BRANCH"
git branch "$BACKUP_BRANCH"

# Reset to target commit
echo "Resetting to target commit..."
git reset --hard "$TARGET_COMMIT"

echo "Rebase complete!"
echo "Large file ./level-crates/rust-ecosystem-level2-0/src/lib.rs has been removed from history"
echo "Backup branch created: $BACKUP_BRANCH"
echo ""
echo "To push changes (WARNING: This rewrites history):"
echo "git push --force-with-lease origin $CURRENT_BRANCH"
