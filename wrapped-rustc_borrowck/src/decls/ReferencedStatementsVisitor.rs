macro_rules! ReferencedStatementsVisitor {
    () => {
        # [doc = " Detect whether one of the provided spans is a statement nested within the top-most visited expr"] struct ReferencedStatementsVisitor < 'a > (& 'a [Span]) ;
    };
}

ReferencedStatementsVisitor!();