// Generated macro for range (function)
macro_rules! Depcrate_testsrange {
() => {
// Module: crate::tests
// Provides: {"range"}
// Dependencies: {}
fn range < 'a > (doc : & 'a [char] , offset : & mut usize , text : & str) -> Range < 'a > { let len = text . chars () . count () ; let range = Range { doc , offset : * offset , len , } ; * offset += len ; range }
};
}
