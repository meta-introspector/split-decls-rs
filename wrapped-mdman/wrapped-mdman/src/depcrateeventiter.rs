// Generated macro for EventIter (type)
macro_rules! DepcrateEventIter {
() => {
// Module: crate
// Provides: {"EventIter"}
// Dependencies: {}
# [doc = " Pulldown-cmark iterator yielding an `(event, range)` tuple."] type EventIter < 'a > = Box < dyn Iterator < Item = (Event < 'a > , Range < usize >) > + 'a > ;
};
}
