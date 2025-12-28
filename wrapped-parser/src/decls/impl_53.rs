macro_rules! deps {
    () => {
        Input!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        # [doc = " `pub` impl used by callers to create `Tokens`."] impl Input { # [inline] pub fn with_capacity (capacity : usize) -> Self { Self { kind : Vec :: with_capacity (capacity) , joint : Vec :: with_capacity (capacity / size_of :: < bits > ()) , contextual_kind : Vec :: with_capacity (capacity) , } } # [inline] pub fn push (& mut self , kind : SyntaxKind) { self . push_impl (kind , SyntaxKind :: EOF) } # [inline] pub fn push_ident (& mut self , contextual_kind : SyntaxKind) { self . push_impl (SyntaxKind :: IDENT , contextual_kind) } # [doc = " Sets jointness for the last token we've pushed."] # [doc = ""] # [doc = " This is a separate API rather than an argument to the `push` to make it"] # [doc = " convenient both for textual and mbe tokens. With text, you know whether"] # [doc = " the *previous* token was joint, with mbe, you know whether the *current*"] # [doc = " one is joint. This API allows for styles of usage:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " // In text:"] # [doc = " tokens.was_joint(prev_joint);"] # [doc = " tokens.push(curr);"] # [doc = ""] # [doc = " // In MBE:"] # [doc = " token.push(curr);"] # [doc = " tokens.push(curr_joint)"] # [doc = " ```"] # [inline] pub fn was_joint (& mut self) { let n = self . len () - 1 ; let (idx , b_idx) = self . bit_index (n) ; self . joint [idx] |= 1 << b_idx ; } # [inline] fn push_impl (& mut self , kind : SyntaxKind , contextual_kind : SyntaxKind) { let idx = self . len () ; if idx . is_multiple_of (bits :: BITS as usize) { self . joint . push (0) ; } self . kind . push (kind) ; self . contextual_kind . push (contextual_kind) ; } }
    };
}

impl_53!();