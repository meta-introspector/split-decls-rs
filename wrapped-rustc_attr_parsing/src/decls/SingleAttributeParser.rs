macro_rules! deps {
    () => {
        Stage!();
        AttributeOrder!();
        AttributeParser!();
        AcceptContext!();
        OnDuplicate!();
        ArgParser!();
        AllowedTargets!();
    };
}

macro_rules! SingleAttributeParser {
    () => {
        deps!();
        # [doc = " Alternative to [`AttributeParser`] that automatically handles state management."] # [doc = " A slightly simpler and more restricted way to convert attributes."] # [doc = " Assumes that an attribute can only appear a single time on an item,"] # [doc = " and errors when it sees more."] # [doc = ""] # [doc = " [`Single<T> where T: SingleAttributeParser`](Single) implements [`AttributeParser`]."] # [doc = ""] # [doc = " [`SingleAttributeParser`] can only convert attributes one-to-one, and cannot combine multiple"] # [doc = " attributes together like is necessary for `#[stable()]` and `#[unstable()]` for example."] pub (crate) trait SingleAttributeParser < S : Stage > : 'static { # [doc = " The single path of the attribute this parser accepts."] # [doc = ""] # [doc = " If you need the parser to accept more than one path, use [`AttributeParser`] instead"] const PATH : & [Symbol] ; # [doc = " Configures the precedence of attributes with the same `PATH` on a syntax node."] const ATTRIBUTE_ORDER : AttributeOrder ; # [doc = " Configures what to do when when the same attribute is"] # [doc = " applied more than once on the same syntax node."] # [doc = ""] # [doc = " [`ATTRIBUTE_ORDER`](Self::ATTRIBUTE_ORDER) specified which one is assumed to be correct,"] # [doc = " and this specified whether to, for example, warn or error on the other one."] const ON_DUPLICATE : OnDuplicate < S > ; const ALLOWED_TARGETS : AllowedTargets ; # [doc = " The template this attribute parser should implement. Used for diagnostics."] const TEMPLATE : AttributeTemplate ; const TYPE : AttributeType = AttributeType :: Normal ; # [doc = " Converts a single syntactical attribute to a single semantic attribute, or [`AttributeKind`]"] fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > ; }
    };
}

SingleAttributeParser!();