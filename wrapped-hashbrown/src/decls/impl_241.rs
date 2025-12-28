macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < K , V , S , A > PartialEq for HashMap < K , V , S , A > where K : Eq + Hash , V : PartialEq , S : BuildHasher , A : Allocator , { fn eq (& self , other : & Self) -> bool { if self . len () != other . len () { return false ; } self . iter () . all (| (key , value) | other . get (key) . map_or (false , | v | * value == * v)) } }
    };
}

impl_241!();