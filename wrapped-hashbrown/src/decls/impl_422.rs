macro_rules! deps {
    () => {
        IntoIter!();
        HashSet!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl < T , S , A : Allocator > IntoIterator for HashSet < T , S , A > { type Item = T ; type IntoIter = IntoIter < T , A > ; # [doc = " Creates a consuming iterator, that is, one that moves each value out"] # [doc = " of the set in arbitrary order. The set cannot be used after calling"] # [doc = " this."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = " let mut set = HashSet::new();"] # [doc = " set.insert(\"a\".to_string());"] # [doc = " set.insert(\"b\".to_string());"] # [doc = ""] # [doc = " // Not possible to collect to a Vec<String> with a regular `.iter()`."] # [doc = " let v: Vec<String> = set.into_iter().collect();"] # [doc = ""] # [doc = " // Will print in an arbitrary order."] # [doc = " for x in &v {"] # [doc = "     println!(\"{}\", x);"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] fn into_iter (self) -> IntoIter < T , A > { IntoIter { iter : self . map . into_iter () , } } }
    };
}

impl_422!()