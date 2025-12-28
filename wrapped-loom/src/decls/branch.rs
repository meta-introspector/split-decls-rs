macro_rules! deps {
    () => {
        Scheduler!();
        Execution!();
    };
}

macro_rules! branch {
    () => {
        deps!();
        # [doc = " Add an execution branch point."] fn branch < F , R > (f : F) -> R where F : FnOnce (& mut Execution) -> R , { let (ret , switch) = execution (| execution | { let ret = f (execution) ; let switch = execution . schedule () ; trace ! (? switch , "branch") ; (ret , switch) }) ; if switch { Scheduler :: switch () ; } ret }
    };
}

branch!()