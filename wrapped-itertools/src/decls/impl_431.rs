macro_rules! deps {
    () => {
        Permutations!();
        PermutationState!();
        SizeHint!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl < I > Iterator for Permutations < I > where I : Iterator , I :: Item : Clone , { type Item = Vec < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { let Self { vals , state } = self ; match state { PermutationState :: Start { k : 0 } => { * state = PermutationState :: End ; Some (Vec :: new ()) } & mut PermutationState :: Start { k } => { vals . prefill (k) ; if vals . len () != k { * state = PermutationState :: End ; return None ; } * state = PermutationState :: Buffered { k , min_n : k } ; Some (vals [0 .. k] . to_vec ()) } PermutationState :: Buffered { ref k , min_n } => { if vals . get_next () { let item = (0 .. * k - 1) . chain (once (* min_n)) . map (| i | vals [i] . clone ()) . collect () ; * min_n += 1 ; Some (item) } else { let n = * min_n ; let prev_iteration_count = n - * k + 1 ; let mut indices : Box < [_] > = (0 .. n) . collect () ; let mut cycles : Box < [_] > = (n - k .. n) . rev () . collect () ; for _ in 0 .. prev_iteration_count { if advance (& mut indices , & mut cycles) { * state = PermutationState :: End ; return None ; } } let item = vals . get_at (& indices [0 .. * k]) ; * state = PermutationState :: Loaded { indices , cycles } ; Some (item) } } PermutationState :: Loaded { indices , cycles } => { if advance (indices , cycles) { * state = PermutationState :: End ; return None ; } let k = cycles . len () ; Some (vals . get_at (& indices [0 .. k])) } PermutationState :: End => None , } } fn count (self) -> usize { let Self { vals , state } = self ; let n = vals . count () ; state . size_hint_for (n) . 1 . unwrap () } fn size_hint (& self) -> SizeHint { let (mut low , mut upp) = self . vals . size_hint () ; low = self . state . size_hint_for (low) . 0 ; upp = upp . and_then (| n | self . state . size_hint_for (n) . 1) ; (low , upp) } }
    };
}

impl_431!()