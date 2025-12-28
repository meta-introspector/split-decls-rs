macro_rules! deps {
    () => {
        TopSubtree!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < S : Copy > TopSubtree < S > { pub (crate) fn top_subtree (& self) -> & tt :: Subtree < S > { let tt :: TokenTree :: Subtree (subtree) = & self . 0 [0] else { unreachable ! ("the first token tree is always the top subtree") ; } ; subtree } pub (crate) fn from_bridge (group : bridge :: Group < TokenStream < S > , S >) -> Self { let delimiter = delim_to_internal (group . delimiter , group . span) ; let mut tts = group . stream . map (| it | it . token_trees) . unwrap_or_else (| | Vec :: with_capacity (1)) ; tts . insert (0 , tt :: TokenTree :: Subtree (tt :: Subtree { delimiter , len : tts . len () as u32 })) ; TopSubtree (tts) } }
    };
}

impl_27!();