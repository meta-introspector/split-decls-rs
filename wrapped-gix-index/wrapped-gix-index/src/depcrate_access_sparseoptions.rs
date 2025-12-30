// Generated macro for Options (struct)
macro_rules! Depcrate_access_sparseOptions {
() => {
// Module: crate::access::sparse
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Configuration related to sparse indexes."] # [derive (Debug , Default , Clone , Copy)] pub struct Options { # [doc = " If true, certain entries in the index will be excluded / skipped for certain operations,"] # [doc = " based on the ignore patterns in the `.git/info/sparse-checkout` file. These entries will"] # [doc = " carry the [`SKIP_WORKTREE`][crate::entry::Flags::SKIP_WORKTREE] flag."] # [doc = ""] # [doc = " This typically is the value of `core.sparseCheckout` in the git configuration."] pub sparse_checkout : bool , # [doc = " Interpret the `.git/info/sparse-checkout` file using _cone mode_."] # [doc = ""] # [doc = " If true, _cone mode_ is active and entire directories will be included in the checkout, as well as files in the root"] # [doc = " of the repository."] # [doc = " If false, non-cone mode is active and entries to _include_ will be matched with patterns like those found in `.gitignore` files."] # [doc = ""] # [doc = " This typically is the value of `core.sparseCheckoutCone` in the git configuration."] pub directory_patterns_only : bool , # [doc = " If true, will attempt to write a sparse index file which only works in cone mode."] # [doc = ""] # [doc = " A sparse index has [`DIR` entries][crate::entry::Mode::DIR] that represent entire directories to be skipped"] # [doc = " during checkout and other operations due to the added presence of"] # [doc = " the [`SKIP_WORKTREE`][crate::entry::Flags::SKIP_WORKTREE] flag."] # [doc = ""] # [doc = " This is typically the value of `index.sparse` in the git configuration."] pub write_sparse_index : bool , }
};
}
