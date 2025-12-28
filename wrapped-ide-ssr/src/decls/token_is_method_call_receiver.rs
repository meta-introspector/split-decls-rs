macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! token_is_method_call_receiver {
    () => {
        deps!();
        # [doc = " Returns whether token is the receiver of a method call. Note, being within the receiver of a"] # [doc = " method call doesn't count. e.g. if the token is `$a`, then `$a.foo()` will return true, while"] # [doc = " `($a + $b).foo()` or `x.foo($a)` will return false."] fn token_is_method_call_receiver (token : & SyntaxToken) -> bool { if let Some (receiver) = token . parent_ancestors () . find_map (ast :: MethodCallExpr :: cast) . and_then (| call | call . receiver ()) { let tokens = receiver . syntax () . descendants_with_tokens () . filter_map (| node_or_token | { match node_or_token { SyntaxElement :: Token (t) => Some (t) , _ => None , } }) ; if let Some ((only_token ,)) = tokens . collect_tuple () { return only_token == * token ; } } false }
    };
}

token_is_method_call_receiver!()