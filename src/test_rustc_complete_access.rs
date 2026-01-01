// Test case to diagnose rustc_complete module visibility issue

use crate::rustc_complete::ty::Ty;

fn test_rustc_complete_access() {
    // This should work if rustc_complete::ty is properly accessible
    let _test: Ty<i32> = Ty(42);
}
