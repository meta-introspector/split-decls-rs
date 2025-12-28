macro_rules! IdIterator {
    () => {
        # [derive (Debug , Clone)] struct IdIterator < 'a , S > { upper_bound : usize , removed_ids : & 'a IndexSet < usize , S > , current : Option < usize > , }
    };
}

IdIterator!()