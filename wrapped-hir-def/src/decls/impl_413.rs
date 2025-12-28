macro_rules! deps {
    () => {
        Choice!();
        Stability!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl Choice { fn new (prefer_prelude : bool , kind : PathKind , name : Name , stability : Stability) -> Self { Self { path_text_len : path_kind_len (kind) + name . as_str () . len () , stability , prefer_due_to_prelude : prefer_prelude && name == sym :: prelude , path : ModPath :: from_segments (kind , iter :: once (name)) , } } fn push (mut self , prefer_prelude : bool , name : Name) -> Self { self . path_text_len += name . as_str () . len () ; self . prefer_due_to_prelude |= prefer_prelude && name == sym :: prelude ; self . path . push_segment (name) ; self } fn try_select (current : & mut Option < Choice > , mut other : Choice , prefer_prelude : bool , name : Name ,) { let Some (current) = current else { * current = Some (other . push (prefer_prelude , name)) ; return ; } ; match other . stability . cmp (& current . stability) . then_with (| | other . prefer_due_to_prelude . cmp (& current . prefer_due_to_prelude)) . then_with (| | (current . path . len ()) . cmp (& (other . path . len () + 1))) { Ordering :: Less => return , Ordering :: Equal => { other . path_text_len += name . as_str () . len () ; if let Ordering :: Less | Ordering :: Equal = current . path_text_len . cmp (& other . path_text_len) { return ; } } Ordering :: Greater => { other . path_text_len += name . as_str () . len () ; } } other . path . push_segment (name) ; * current = other ; } }
    };
}

impl_413!()