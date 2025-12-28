macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Event {
    () => {
        deps!();
        # [doc = " `Parser` produces a flat list of `Event`s."] # [doc = " They are converted to a tree-structure in"] # [doc = " a separate pass, via `TreeBuilder`."] # [derive (Debug , PartialEq)] pub (crate) enum Event { # [doc = " This event signifies the start of the node."] # [doc = " It should be either abandoned (in which case the"] # [doc = " `kind` is `TOMBSTONE`, and the event is ignored),"] # [doc = " or completed via a `Finish` event."] # [doc = ""] # [doc = " All tokens between a `Start` and a `Finish` would"] # [doc = " become the children of the respective node."] # [doc = ""] # [doc = " For left-recursive syntactic constructs, the parser produces"] # [doc = " a child node before it sees a parent. `forward_parent`"] # [doc = " saves the position of current event's parent."] # [doc = ""] # [doc = " Consider this path"] # [doc = ""] # [doc = " foo::bar"] # [doc = ""] # [doc = " The events for it would look like this:"] # [doc = ""] # [doc = " ```text"] # [doc = " START(PATH) IDENT('foo') FINISH START(PATH) T![::] IDENT('bar') FINISH"] # [doc = "       |                          /\\"] # [doc = "       |                          |"] # [doc = "       +------forward-parent------+"] # [doc = " ```"] # [doc = ""] # [doc = " And the tree would look like this"] # [doc = ""] # [doc = " ```text"] # [doc = "    +--PATH---------+"] # [doc = "    |   |           |"] # [doc = "    |   |           |"] # [doc = "    |  '::'       'bar'"] # [doc = "    |"] # [doc = "   PATH"] # [doc = "    |"] # [doc = "   'foo'"] # [doc = " ```"] # [doc = ""] # [doc = " See also `CompletedMarker::precede`."] Start { kind : SyntaxKind , forward_parent : Option < u32 > , } , # [doc = " Complete the previous `Start` event"] Finish , # [doc = " Produce a single leaf-element."] # [doc = " `n_raw_tokens` is used to glue complex contextual tokens."] # [doc = " For example, lexer tokenizes `>>` as `>`, `>`, and"] # [doc = " `n_raw_tokens = 2` is used to produced a single `>>`."] Token { kind : SyntaxKind , n_raw_tokens : u8 , } , # [doc = " When we parse `foo.0.0` or `foo. 0. 0` the lexer will hand us a float literal"] # [doc = " instead of an integer literal followed by a dot as the lexer has no contextual knowledge."] # [doc = " This event instructs whatever consumes the events to split the float literal into"] # [doc = " the corresponding parts."] FloatSplitHack { ends_in_dot : bool , } , Error { msg : String , } , }
    };
}

Event!();