macro_rules! MalformedDerive {
    () => {
        # [derive (Debug)] pub struct MalformedDerive { pub node : InFile < SyntaxNodePtr > , }
    };
}

MalformedDerive!()