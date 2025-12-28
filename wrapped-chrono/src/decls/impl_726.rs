macro_rules! deps {
    () => {
        Month!();
    };
}

macro_rules! impl_726 {
    () => {
        deps!();
        impl num_traits :: FromPrimitive for Month { # [doc = " Returns an `Option<Month>` from a i64, assuming a 1-index, January = 1."] # [doc = ""] # [doc = " `Month::from_i64(n: i64)`: | `1`                  | `2`                   | ... | `12`"] # [doc = " ---------------------------| -------------------- | --------------------- | ... | -----"] # [doc = " ``:                        | Some(Month::January) | Some(Month::February) | ... | Some(Month::December)"] # [inline] fn from_u64 (n : u64) -> Option < Month > { Self :: from_u32 (n as u32) } # [inline] fn from_i64 (n : i64) -> Option < Month > { Self :: from_u32 (n as u32) } # [inline] fn from_u32 (n : u32) -> Option < Month > { match n { 1 => Some (Month :: January) , 2 => Some (Month :: February) , 3 => Some (Month :: March) , 4 => Some (Month :: April) , 5 => Some (Month :: May) , 6 => Some (Month :: June) , 7 => Some (Month :: July) , 8 => Some (Month :: August) , 9 => Some (Month :: September) , 10 => Some (Month :: October) , 11 => Some (Month :: November) , 12 => Some (Month :: December) , _ => None , } } }
    };
}

impl_726!();