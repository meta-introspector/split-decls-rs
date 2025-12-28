macro_rules! deps {
    () => {
        FactWriter!();
        FactRow!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl < 'w > FactWriter < 'w > { fn write_facts_to_path < T > (& self , rows : & [T] , file_name : & str) -> Result < () , Box < dyn Error > > where T : FactRow , { let file = & self . dir . join (file_name) ; let mut file = File :: create_buffered (file) ? ; for row in rows { row . write (& mut file , self . location_table) ? ; } Ok (()) } }
    };
}

impl_261!()