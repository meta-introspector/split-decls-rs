// Generated macro for tests (module)
macro_rules! Depcrate_mapref_entrytests {
() => {
// Module: crate::mapref::entry
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: DashMap ; use super :: * ; # [test] fn test_insert_into_vacant () { let map : DashMap < u32 , u32 > = DashMap :: new () ; let entry = map . entry (1) ; assert ! (matches ! (entry , Entry :: Vacant (_))) ; let val = entry . insert (2) ; assert_eq ! (* val , 2) ; drop (val) ; assert_eq ! (* map . get (& 1) . unwrap () , 2) ; } # [test] fn test_insert_into_occupied () { let map : DashMap < u32 , u32 > = DashMap :: new () ; map . insert (1 , 1000) ; let entry = map . entry (1) ; assert ! (matches ! (& entry , Entry :: Occupied (entry) if * entry . get () == 1000)) ; let val = entry . insert (2) ; assert_eq ! (* val , 2) ; drop (val) ; assert_eq ! (* map . get (& 1) . unwrap () , 2) ; } # [test] fn test_insert_entry_into_vacant () { let map : DashMap < u32 , u32 > = DashMap :: new () ; let entry = map . entry (1) ; assert ! (matches ! (entry , Entry :: Vacant (_))) ; let entry = entry . insert_entry (2) ; assert_eq ! (* entry . get () , 2) ; drop (entry) ; assert_eq ! (* map . get (& 1) . unwrap () , 2) ; } # [test] fn test_insert_entry_into_occupied () { let map : DashMap < u32 , u32 > = DashMap :: new () ; map . insert (1 , 1000) ; let entry = map . entry (1) ; assert ! (matches ! (& entry , Entry :: Occupied (entry) if * entry . get () == 1000)) ; let entry = entry . insert_entry (2) ; assert_eq ! (* entry . get () , 2) ; drop (entry) ; assert_eq ! (* map . get (& 1) . unwrap () , 2) ; } }
};
}
