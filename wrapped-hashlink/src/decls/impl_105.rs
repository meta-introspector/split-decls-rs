macro_rules! deps {
    () => {
        IntoIter!();
        LinkedHashMap!();
        ValueLinks!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < K , V , S > IntoIterator for LinkedHashMap < K , V , S > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; # [inline] fn into_iter (mut self) -> IntoIter < K , V > { unsafe { let (head , tail) = if let Some (values) = self . values { let ValueLinks { next : head , prev : tail , } = values . as_ref () . links . value ; let _ = Box :: from_raw (self . values . as_ptr ()) ; self . values = None ; (Some (head) , Some (tail)) } else { (None , None) } ; let len = self . len () ; drop_free_nodes (self . free . take ()) ; self . table . clear () ; IntoIter { head , tail , remaining : len , marker : PhantomData , } } } }
    };
}

impl_105!();