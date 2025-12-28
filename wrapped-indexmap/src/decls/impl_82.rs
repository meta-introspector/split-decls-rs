macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        # [doc = " Access [`IndexSet`] values at indexed positions."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::IndexSet;"] # [doc = ""] # [doc = " let mut set = IndexSet::new();"] # [doc = " for word in \"Lorem ipsum dolor sit amet\".split_whitespace() {"] # [doc = "     set.insert(word.to_string());"] # [doc = " }"] # [doc = " assert_eq!(set[0], \"Lorem\");"] # [doc = " assert_eq!(set[1], \"ipsum\");"] # [doc = " set.reverse();"] # [doc = " assert_eq!(set[0], \"amet\");"] # [doc = " assert_eq!(set[1], \"sit\");"] # [doc = " set.sort();"] # [doc = " assert_eq!(set[0], \"Lorem\");"] # [doc = " assert_eq!(set[1], \"amet\");"] # [doc = " ```"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use indexmap::IndexSet;"] # [doc = ""] # [doc = " let mut set = IndexSet::new();"] # [doc = " set.insert(\"foo\");"] # [doc = " println!(\"{:?}\", set[10]); // panics!"] # [doc = " ```"] impl < T , S > Index < usize > for IndexSet < T , S > { type Output = T ; # [doc = " Returns a reference to the value at the supplied `index`."] # [doc = ""] # [doc = " ***Panics*** if `index` is out of bounds."] fn index (& self , index : usize) -> & T { if let Some (value) = self . get_index (index) { value } else { panic ! ("index out of bounds: the len is {len} but the index is {index}" , len = self . len ()) ; } } }
    };
}

impl_82!()