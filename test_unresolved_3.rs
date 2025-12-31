// Test case for unresolved dependency: rustc_session::EarlyDiagCtxt::new
// Similar matches found in symbol database:
// - rustc_mir_transform::shim::new_body
// - rustc_mir_transform::gvn::macro_call_newtype_index
// - rustc_middle::thir::use_rustc_index___{_IndexVec_,_newtype_index_}

// Expected: use rustc_session::EarlyDiagCtxt;
fn test_new() {
    // Call: rustc_session::EarlyDiagCtxt::new;
    println!("Testing dependency resolution");
}
