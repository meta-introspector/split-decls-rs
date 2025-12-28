macro_rules! deps {
    () => {
        DotDotPos!();
        RangeEnd!();
        QPath!();
        Expr!();
        Pat!();
        PatExpr!();
        Variant!();
        PatField!();
    };
}

macro_rules! PatKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum PatKind < 'hir > { # [doc = " A missing pattern, e.g. for an anonymous param in a bare fn like `fn f(u32)`."] Missing , # [doc = " Represents a wildcard pattern (i.e., `_`)."] Wild , # [doc = " A fresh binding `ref mut binding @ OPT_SUBPATTERN`."] # [doc = " The `HirId` is the canonical ID for the variable being bound,"] # [doc = " (e.g., in `Ok(x) | Err(x)`, both `x` use the same canonical ID),"] # [doc = " which is the pattern ID of the first `x`."] # [doc = ""] # [doc = " The `BindingMode` is what's provided by the user, before match"] # [doc = " ergonomics are applied. For the binding mode actually in use,"] # [doc = " see [`TypeckResults::extract_binding_mode`]."] # [doc = ""] # [doc = " [`TypeckResults::extract_binding_mode`]: ../../rustc_middle/ty/struct.TypeckResults.html#method.extract_binding_mode"] Binding (BindingMode , HirId , Ident , Option < & 'hir Pat < 'hir > >) , # [doc = " A struct or struct variant pattern (e.g., `Variant {x, y, ..}`)."] # [doc = " The `Option` contains the span of a possible `..`."] Struct (QPath < 'hir > , & 'hir [PatField < 'hir >] , Option < Span >) , # [doc = " A tuple struct/variant pattern `Variant(x, y, .., z)`."] # [doc = " If the `..` pattern fragment is present, then `DotDotPos` denotes its position."] # [doc = " `0 <= position <= subpats.len()`"] TupleStruct (QPath < 'hir > , & 'hir [Pat < 'hir >] , DotDotPos) , # [doc = " An or-pattern `A | B | C`."] # [doc = " Invariant: `pats.len() >= 2`."] Or (& 'hir [Pat < 'hir >]) , # [doc = " A never pattern `!`."] Never , # [doc = " A tuple pattern (e.g., `(a, b)`)."] # [doc = " If the `..` pattern fragment is present, then `DotDotPos` denotes its position."] # [doc = " `0 <= position <= subpats.len()`"] Tuple (& 'hir [Pat < 'hir >] , DotDotPos) , # [doc = " A `box` pattern."] Box (& 'hir Pat < 'hir >) , # [doc = " A `deref` pattern (currently `deref!()` macro-based syntax)."] Deref (& 'hir Pat < 'hir >) , # [doc = " A reference pattern (e.g., `&mut (a, b)`)."] Ref (& 'hir Pat < 'hir > , Mutability) , # [doc = " A literal, const block or path."] Expr (& 'hir PatExpr < 'hir >) , # [doc = " A guard pattern (e.g., `x if guard(x)`)."] Guard (& 'hir Pat < 'hir > , & 'hir Expr < 'hir >) , # [doc = " A range pattern (e.g., `1..=2` or `1..2`)."] Range (Option < & 'hir PatExpr < 'hir > > , Option < & 'hir PatExpr < 'hir > > , RangeEnd) , # [doc = " A slice pattern, `[before_0, ..., before_n, (slice, after_0, ..., after_n)?]`."] # [doc = ""] # [doc = " Here, `slice` is lowered from the syntax `($binding_mode $ident @)? ..`."] # [doc = " If `slice` exists, then `after` can be non-empty."] # [doc = ""] # [doc = " The representation for e.g., `[a, b, .., c, d]` is:"] # [doc = " ```ignore (illustrative)"] # [doc = " PatKind::Slice([Binding(a), Binding(b)], Some(Wild), [Binding(c), Binding(d)])"] # [doc = " ```"] Slice (& 'hir [Pat < 'hir >] , Option < & 'hir Pat < 'hir > > , & 'hir [Pat < 'hir >]) , # [doc = " A placeholder for a pattern that wasn't well formed in some way."] Err (ErrorGuaranteed) , }
    };
}

PatKind!()