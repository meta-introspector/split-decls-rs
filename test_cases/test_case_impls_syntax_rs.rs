// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_query_system/src/ich/impls_syntax.rs
// Error: expected square brackets
// Problematic line: line 14

impl<'ctx> rustc_abi::HashStableContext for StableHashingContext<'ctx> {}
impl<'ctx> rustc_ast::HashStableContext for StableHashingContext<'ctx> {}

impl<'a> HashStable<StableHashingContext<'a>> for [hir::Attribute] {
    fn hash_stable(&self, hcx: &mut StableHashingContext<'a>, hasher: &mut StableHasher) {
        if self.is_empty() {
            self.len().hash_stable(hcx, hasher);
