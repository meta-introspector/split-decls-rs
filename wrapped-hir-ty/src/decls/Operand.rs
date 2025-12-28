macro_rules! deps {
    () => {
        OperandKind!();
        MirSpan!();
    };
}

macro_rules! Operand {
    () => {
        deps!();
        # [doc = " An operand in MIR represents a \"value\" in Rust, the definition of which is undecided and part of"] # [doc = " the memory model. One proposal for a definition of values can be found [on UCG][value-def]."] # [doc = ""] # [doc = " [value-def]: https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/value-domain.md"] # [doc = ""] # [doc = " The most common way to create values is via loading a place. Loading a place is an operation"] # [doc = " which reads the memory of the place and converts it to a value. This is a fundamentally *typed*"] # [doc = " operation. The nature of the value produced depends on the type of the conversion. Furthermore,"] # [doc = " there may be other effects: if the type has a validity constraint loading the place might be UB"] # [doc = " if the validity constraint is not met."] # [doc = ""] # [doc = " **Needs clarification:** Ralf proposes that loading a place not have side-effects."] # [doc = " This is what is implemented in miri today. Are these the semantics we want for MIR? Is this"] # [doc = " something we can even decide without knowing more about Rust's memory model?"] # [doc = ""] # [doc = " **Needs clarification:** Is loading a place that has its variant index set well-formed? Miri"] # [doc = " currently implements it, but it seems like this may be something to check against in the"] # [doc = " validator."] # [derive (Debug , PartialEq , Eq , Clone)] pub struct Operand < 'db > { kind : OperandKind < 'db > , span : Option < MirSpan > , }
    };
}

Operand!();