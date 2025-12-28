macro_rules! deps {
    () => {
        Thread!();
        State!();
        Ref!();
    };
}

macro_rules! Schedule {
    () => {
        deps!();
        # [derive (Debug)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (crate) struct Schedule { # [doc = " Number of times the thread leading to this branch point has been"] # [doc = " pre-empted."] preemptions : u8 , # [doc = " The thread that was active first"] initial_active : Option < u8 > , # [doc = " State of each thread"] threads : [Thread ; MAX_THREADS] , # [doc = " The previous schedule branch"] prev : Option < object :: Ref < Schedule > > , exploring : bool , }
    };
}

Schedule!()