macro_rules! OnClosureNote {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum OnClosureNote < 'a > { # [note (borrowck_closure_invoked_twice)] InvokedTwice { place_name : & 'a str , # [primary_span] span : Span , } , # [note (borrowck_closure_moved_twice)] MovedTwice { place_name : & 'a str , # [primary_span] span : Span , } , }
    };
}

OnClosureNote!();