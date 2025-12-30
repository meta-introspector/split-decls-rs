// Generated macro for tests (module)
macro_rules! Depcrate_rowtests {
() => {
// Module: crate::row
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { Connection , Result } ; # [test] fn test_try_from_row_for_tuple_1 () -> Result < () > { use crate :: ToSql ; use std :: convert :: TryFrom ; let conn = Connection :: open_in_memory () ? ; conn . execute ("CREATE TABLE test (a INTEGER)" , crate :: params_from_iter (std :: iter :: empty :: < & dyn ToSql > ()) ,) ? ; conn . execute ("INSERT INTO test VALUES (42)" , []) ? ; let val = conn . query_row ("SELECT a FROM test" , [] , | row | < (u32 ,) > :: try_from (row)) ? ; assert_eq ! (val , (42 ,)) ; let fail = conn . query_row ("SELECT a FROM test" , [] , | row | < (u32 , u32) > :: try_from (row)) ; fail . unwrap_err () ; Ok (()) } # [test] fn test_try_from_row_for_tuple_2 () -> Result < () > { use std :: convert :: TryFrom ; let conn = Connection :: open_in_memory () ? ; conn . execute ("CREATE TABLE test (a INTEGER, b INTEGER)" , []) ? ; conn . execute ("INSERT INTO test VALUES (42, 47)" , []) ? ; let val = conn . query_row ("SELECT a, b FROM test" , [] , | row | { < (u32 , u32) > :: try_from (row) }) ? ; assert_eq ! (val , (42 , 47)) ; let fail = conn . query_row ("SELECT a, b FROM test" , [] , | row | { < (u32 , u32 , u32) > :: try_from (row) }) ; fail . unwrap_err () ; Ok (()) } # [test] fn test_try_from_row_for_tuple_16 () -> Result < () > { use std :: convert :: TryFrom ; let create_table = "CREATE TABLE test (
            a INTEGER,
            b INTEGER,
            c INTEGER,
            d INTEGER,
            e INTEGER,
            f INTEGER,
            g INTEGER,
            h INTEGER,
            i INTEGER,
            j INTEGER,
            k INTEGER,
            l INTEGER,
            m INTEGER,
            n INTEGER,
            o INTEGER,
            p INTEGER
        )" ; let insert_values = "INSERT INTO test VALUES (
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
            13,
            14,
            15
        )" ; type BigTuple = (u32 , u32 , u32 , u32 , u32 , u32 , u32 , u32 , u32 , u32 , u32 , u32 , u32 , u32 , u32 , u32 ,) ; let conn = Connection :: open_in_memory () ? ; conn . execute (create_table , []) ? ; conn . execute (insert_values , []) ? ; let val = conn . query_row ("SELECT * FROM test" , [] , | row | BigTuple :: try_from (row)) ? ; assert_eq ! (val . 0 , 0) ; assert_eq ! (val . 1 , 1) ; assert_eq ! (val . 2 , 2) ; assert_eq ! (val . 3 , 3) ; assert_eq ! (val . 4 , 4) ; assert_eq ! (val . 5 , 5) ; assert_eq ! (val . 6 , 6) ; assert_eq ! (val . 7 , 7) ; assert_eq ! (val . 8 , 8) ; assert_eq ! (val . 9 , 9) ; assert_eq ! (val . 10 , 10) ; assert_eq ! (val . 11 , 11) ; assert_eq ! (val . 12 , 12) ; assert_eq ! (val . 13 , 13) ; assert_eq ! (val . 14 , 14) ; assert_eq ! (val . 15 , 15) ; Ok (()) } # [test] # [cfg (feature = "bundled")] fn pathological_case () -> Result < () > { let conn = Connection :: open_in_memory () ? ; conn . execute_batch ("CREATE TABLE foo(x);
        CREATE TRIGGER oops BEFORE INSERT ON foo BEGIN SELECT RAISE(FAIL, 'Boom'); END;" ,) ? ; let mut stmt = conn . prepare ("INSERT INTO foo VALUES (0) RETURNING rowid;") ? ; { let iterator_count = stmt . query_map ([] , | _ | Ok (())) ? . count () ; assert_eq ! (1 , iterator_count) ; use fallible_streaming_iterator :: FallibleStreamingIterator ; let fallible_iterator_count = stmt . query ([]) ? . count () . unwrap_or (0) ; assert_eq ! (0 , fallible_iterator_count) ; } { let iterator_last = stmt . query_map ([] , | _ | Ok (())) ? . last () ; assert ! (iterator_last . is_some ()) ; use fallible_iterator :: FallibleIterator ; let fallible_iterator_last = stmt . query ([]) ? . map (| _ | Ok (())) . last () ; assert ! (fallible_iterator_last . is_err ()) ; } Ok (()) } # [test] fn as_ref () -> Result < () > { let conn = Connection :: open_in_memory () ? ; let mut stmt = conn . prepare ("SELECT 'Lisa' as name, 1 as id") ? ; let rows = stmt . query ([]) ? ; assert_eq ! (rows . as_ref () . unwrap () . column_count () , 2) ; Ok (()) } # [test] fn debug () -> Result < () > { let conn = Connection :: open_in_memory () ? ; let mut stmt = conn . prepare ("SELECT 'Lisa' as name, 1 as id, 3.14 as pi, X'53514C697465' as blob, NULL as void" ,) ? ; let mut rows = stmt . query ([]) ? ; let row = rows . next () ? . unwrap () ; let s = format ! ("{row:?}") ; assert_eq ! (s , r#"{"name": (Text, "Lisa"), "id": (Integer, 1), "pi": (Real, 3.14), "blob": (Blob, 6), "void": (Null, ())}"#) ; Ok (()) } }
};
}
