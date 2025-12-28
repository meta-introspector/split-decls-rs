macro_rules! AUTODEREF_RECURSION_LIMIT {
    () => {
        const AUTODEREF_RECURSION_LIMIT : usize = 20 ;
    };
}

AUTODEREF_RECURSION_LIMIT!();