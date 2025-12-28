macro_rules! deps {
    () => {
        Patchwork!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Patchwork { fn new (text : String) -> Patchwork { Patchwork { text , indels : Vec :: new () } } fn patch (& mut self , mut range : Range < usize > , patch : & str) { self . indels . push ((range . clone () , patch . len ())) ; self . indels . sort_by_key (| (delete , _insert) | delete . start) ; let (delete , insert) = self . indels . iter () . take_while (| (delete , _) | delete . start < range . start) . map (| (delete , insert) | (delete . end - delete . start , insert)) . fold ((0usize , 0usize) , | (x1 , y1) , (x2 , y2) | (x1 + x2 , y1 + y2)) ; for pos in & mut [& mut range . start , & mut range . end] { * * pos -= delete ; * * pos += insert ; } self . text . replace_range (range , & patch) ; } }
    };
}

impl_21!();