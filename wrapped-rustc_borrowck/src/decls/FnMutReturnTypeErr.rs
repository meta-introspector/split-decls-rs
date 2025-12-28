macro_rules! FnMutReturnTypeErr {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum FnMutReturnTypeErr { # [label (borrowck_returned_closure_escaped)] ReturnClosure { # [primary_span] span : Span , } , # [label (borrowck_returned_async_block_escaped)] ReturnAsyncBlock { # [primary_span] span : Span , } , # [label (borrowck_returned_ref_escaped)] ReturnRef { # [primary_span] span : Span , } , }
    };
}

FnMutReturnTypeErr!()