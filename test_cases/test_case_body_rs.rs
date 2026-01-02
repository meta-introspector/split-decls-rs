// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_attr_parsing/src/attributes/body.rs
// Error: expected square brackets
// Problematic line: line 7


pub(crate) struct CoroutineParser;

impl<S: Stage> NoArgsAttributeParser<S> for CoroutineParser {
    const PATH: &[Symbol] = &[sym::coroutine];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Error;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Closure)]);
