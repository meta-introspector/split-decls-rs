macro_rules! deps {
    () => {
        Stage!();
        AttributeParser!();
        Early!();
        Late!();
    };
}

macro_rules! SharedContext {
    () => {
        deps!();
        # [doc = " Context given to every attribute parser during finalization."] # [doc = ""] # [doc = " Gives [`AttributeParser`](crate::attributes::AttributeParser)s enough information to create"] # [doc = " errors, for example."] pub struct SharedContext < 'p , 'sess , S : Stage > { # [doc = " The parse context, gives access to the session and the"] # [doc = " diagnostics context."] pub (crate) cx : & 'p mut AttributeParser < 'sess , S > , # [doc = " The span of the syntactical component this attribute was applied to"] pub (crate) target_span : Span , # [doc = " The id ([`NodeId`] if `S` is `Early`, [`HirId`] if `S` is `Late`) of the syntactical component this attribute was applied to"] pub (crate) target_id : S :: Id , pub (crate) emit_lint : & 'p mut dyn FnMut (AttributeLint < S :: Id >) , }
    };
}

SharedContext!()