// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , crate :: debug_account_data :: { Hex , MAX_DEBUG_ACCOUNT_DATA } , } ; # [test] fn test_next_account_infos () { let k1 = Address :: new_unique () ; let k2 = Address :: new_unique () ; let k3 = Address :: new_unique () ; let k4 = Address :: new_unique () ; let k5 = Address :: new_unique () ; let l1 = & mut 0 ; let l2 = & mut 0 ; let l3 = & mut 0 ; let l4 = & mut 0 ; let l5 = & mut 0 ; let d1 = & mut [0u8] ; let d2 = & mut [0u8] ; let d3 = & mut [0u8] ; let d4 = & mut [0u8] ; let d5 = & mut [0u8] ; let infos = & [AccountInfo :: new (& k1 , false , false , l1 , d1 , & k1 , false) , AccountInfo :: new (& k2 , false , false , l2 , d2 , & k2 , false) , AccountInfo :: new (& k3 , false , false , l3 , d3 , & k3 , false) , AccountInfo :: new (& k4 , false , false , l4 , d4 , & k4 , false) , AccountInfo :: new (& k5 , false , false , l5 , d5 , & k5 , false) ,] ; let infos_iter = & mut infos . iter () ; let info1 = next_account_info (infos_iter) . unwrap () ; let info2_3_4 = next_account_infos (infos_iter , 3) . unwrap () ; let info5 = next_account_info (infos_iter) . unwrap () ; assert_eq ! (k1 , * info1 . key) ; assert_eq ! (k2 , * info2_3_4 [0] . key) ; assert_eq ! (k3 , * info2_3_4 [1] . key) ; assert_eq ! (k4 , * info2_3_4 [2] . key) ; assert_eq ! (k5 , * info5 . key) ; } # [test] fn test_account_info_as_ref () { let k = Address :: new_unique () ; let l = & mut 0 ; let d = & mut [0u8] ; let info = AccountInfo :: new (& k , false , false , l , d , & k , false) ; assert_eq ! (info . key , info . as_ref () . key) ; } # [test] fn test_account_info_debug_data () { let key = Address :: new_unique () ; let mut lamports = 42 ; let mut data = vec ! [5 ; 80] ; let data_str = format ! ("{:?}" , Hex (& data [.. MAX_DEBUG_ACCOUNT_DATA])) ; let info = AccountInfo :: new (& key , false , false , & mut lamports , & mut data , & key , false) ; assert_eq ! (format ! ("{info:?}") , format ! ("AccountInfo {{ \
                key: {}, \
                owner: {}, \
                is_signer: {}, \
                is_writable: {}, \
                executable: {}, \
                lamports: {}, \
                data.len: {}, \
                data: {}, .. }}" , key , key , false , false , false , lamports , data . len () , data_str ,)) ; let mut data = vec ! [5 ; 40] ; let data_str = format ! ("{:?}" , Hex (& data)) ; let info = AccountInfo :: new (& key , false , false , & mut lamports , & mut data , & key , false) ; assert_eq ! (format ! ("{info:?}") , format ! ("AccountInfo {{ \
                key: {}, \
                owner: {}, \
                is_signer: {}, \
                is_writable: {}, \
                executable: {}, \
                lamports: {}, \
                data.len: {}, \
                data: {}, .. }}" , key , key , false , false , false , lamports , data . len () , data_str ,)) ; let mut data = vec ! [] ; let info = AccountInfo :: new (& key , false , false , & mut lamports , & mut data , & key , false) ; assert_eq ! (format ! ("{info:?}") , format ! ("AccountInfo {{ \
                key: {}, \
                owner: {}, \
                is_signer: {}, \
                is_writable: {}, \
                executable: {}, \
                lamports: {}, \
                data.len: {}, .. }}" , key , key , false , false , false , lamports , data . len () ,)) ; } # [test] fn test_layout_assumptions () { super :: check_type_assumptions () ; } }
};
}
