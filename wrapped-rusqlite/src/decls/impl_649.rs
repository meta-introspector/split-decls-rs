macro_rules! deps {
    () => {
        Updates!();
        Inserts!();
        VTabLog!();
        ValueRef!();
        UpdateVTab!();
        Result!();
    };
}

macro_rules! impl_649 {
    () => {
        deps!();
        impl UpdateVTab < '_ > for VTabLog { fn delete (& mut self , arg : ValueRef < '_ >) -> Result < () > { println ! ("VTabLog::delete({}, {arg:?})" , self . i_inst) ; Ok (()) } fn insert (& mut self , args : & Inserts < '_ >) -> Result < i64 > { println ! ("VTabLog::insert({}, on_conflict:{:?}, {:?})" , self . i_inst , unsafe { args . on_conflict (self . db) } , args . iter () . collect ::< Vec < ValueRef <'_ >>> ()) ; Ok (self . n_row) } fn update (& mut self , args : & Updates < '_ >) -> Result < () > { println ! ("VTabLog::update({}, on_conflict:{:?}, {:?})" , self . i_inst , unsafe { args . on_conflict (self . db) } , args . iter () . enumerate () . map (| (i , v) | (v , args . no_change (i))) . collect ::< Vec < (ValueRef <'_ >, bool) >> ()) ; Ok (()) } }
    };
}

impl_649!()