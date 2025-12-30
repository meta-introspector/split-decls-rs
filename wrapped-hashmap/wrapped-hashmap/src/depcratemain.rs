// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let map_1m_integers = create_map_1m_integers () ; run_benchmark_group (| group | { group . register_benchmark ("hashmap_insert_1m" , | | { let count = 1_000_000 ; let mut map = HashMap :: with_capacity_and_hasher (count * 2 , FxBuildHasher :: default () ,) ; move | | { for index in 0 .. count { map . insert (index , index) ; } } }) ; group . register_benchmark ("hashmap_remove_1m" , | | { let mut map = create_map_1m_integers () ; move | | { for index in 0 .. map . capacity () { map . remove (& (index as u64)) ; } } }) ; group . register_benchmark ("hashmap_find_1m" , | | { | | { let map = & map_1m_integers ; for index in 0 .. map . capacity () { black_box (map . get (& (index as u64))) ; } } }) ; group . register_benchmark ("hashmap_find_misses_1m" , | | { | | { let map = & map_1m_integers ; for index in map . capacity () .. (map . capacity () * 2) { black_box (map . get (& (index as u64))) ; } } }) ; group . register_benchmark ("hashmap_iterate_1m" , | | { | | map_1m_integers . values () . sum :: < u64 > () }) ; }) ; }
};
}
