macro_rules! deps {
    () => {
        State!();
        Set!();
        LocationSet!();
        UnsafeCell!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl State { fn new (threads : & thread :: Set , location : Location) -> State { let version = threads . active () . causality ; State { created_location : location , is_reading : 0 , is_writing : false , read_access : version , read_locations : LocationSet :: new () , write_access : version , write_locations : LocationSet :: new () , } } # [doc = " Perform a read access"] fn track_read (& mut self , threads : & thread :: Set) { let current = & threads . active () . causality ; if let Some (writer) = current . ahead (& self . write_access) { location :: panic ("Causality violation: Concurrent read and write accesses.") . location ("created" , self . created_location) . thread ("read" , threads . active_id () , self . read_locations [threads]) . thread ("write" , writer , self . write_locations [writer]) . fire () ; } self . read_access . join (current) ; } fn track_write (& mut self , threads : & thread :: Set) { let current = & threads . active () . causality ; if let Some (other) = current . ahead (& self . write_access) { location :: panic ("Causality violation: Concurrent write accesses to `UnsafeCell`.") . location ("created" , self . created_location) . thread ("write one" , other , self . write_locations [other]) . thread ("write two" , threads . active_id () , self . write_locations [threads] ,) . fire () ; } if let Some (reader) = current . ahead (& self . read_access) { location :: panic ("Causality violation: Concurrent read and write accesses to `UnsafeCell`." ,) . location ("created" , self . created_location) . thread ("read" , reader , self . read_locations [reader]) . thread ("write" , threads . active_id () , self . write_locations [threads]) . fire () ; } self . write_access . join (current) ; } }
    };
}

impl_53!();