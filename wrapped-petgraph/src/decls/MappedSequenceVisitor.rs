macro_rules! MappedSequenceVisitor {
    () => {
        # [doc = " Serde combinator. A sequence visitor that maps deserialized elements"] # [doc = " lazily; the visitor can also emit new errors if the elements have errors."] pub struct MappedSequenceVisitor < T , R , F > where F : Fn (T) -> Result < R , & 'static str > , { f : F , marker : PhantomData < fn () -> T > , }
    };
}

MappedSequenceVisitor!();