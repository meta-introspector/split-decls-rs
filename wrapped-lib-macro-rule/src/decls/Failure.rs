macro_rules! Failure {
    () => {
        # [derive (Debug)] pub struct Failure { pub tokens : TokenStream , pub lo : Span , pub hi : Span , }
    };
}

Failure!()