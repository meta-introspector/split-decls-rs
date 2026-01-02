// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_query_system/src/query/config.rs
// Error: expected square brackets
// Problematic line: line 18


pub type HashResult<V> = Option<fn(&mut StableHashingContext<'_>, &V) -> Fingerprint>;

pub trait QueryConfig<Qcx: QueryContext>: Copy {
    fn name(self) -> &'static str;

    // `Key` and `Value` are `Copy` instead of `Clone` to ensure copying them stays cheap,
