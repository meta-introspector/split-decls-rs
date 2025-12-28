macro_rules! StashApplyProgress {
    () => {
        # [allow (missing_docs)] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum StashApplyProgress { # [doc = " None"] None , # [doc = " Loading the stashed data from the object database"] LoadingStash , # [doc = " The stored index is being analyzed"] AnalyzeIndex , # [doc = " The modified files are being analyzed"] AnalyzeModified , # [doc = " The untracked and ignored files are being analyzed"] AnalyzeUntracked , # [doc = " The untracked files are being written to disk"] CheckoutUntracked , # [doc = " The modified files are being written to disk"] CheckoutModified , # [doc = " The stash was applied successfully"] Done , }
    };
}

StashApplyProgress!();