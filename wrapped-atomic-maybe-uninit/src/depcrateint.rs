// Generated macro for int (macro)
macro_rules! Depcrateint {
() => {
// Module: crate
// Provides: {"int"}
// Dependencies: {}
macro_rules ! int { ($ ty : ident , $ align : ident) => { impl raw :: Primitive for $ ty { } const _ : () = { assert ! (mem :: size_of ::< AtomicMaybeUninit <$ ty >> () == mem :: size_of ::<$ ty > ()) ; assert ! (mem :: align_of ::< AtomicMaybeUninit <$ ty >> () == mem :: size_of ::<$ ty > ()) ; } ; unsafe impl private :: PrimitivePriv for $ ty { type Align = private ::$ align ; } impl AtomicMaybeUninit <$ ty > { # [doc = " Creates a new atomic value from a potentially uninitialized value."] # [inline] # [must_use] # [deprecated (since = "0.3.10" , note = "use `new` instead because it is now always `const fn`")] pub const fn const_new (v : MaybeUninit <$ ty >) -> Self { Self { v : UnsafeCell :: new (v) , _align : [] } } } } ; }
};
}
