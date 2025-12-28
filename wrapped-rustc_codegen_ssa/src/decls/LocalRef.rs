macro_rules! deps {
    () => {
        PlaceRef!();
        OperandRef!();
        OperandValue!();
    };
}

macro_rules! LocalRef {
    () => {
        deps!();
        enum LocalRef < 'tcx , V > { Place (PlaceRef < 'tcx , V >) , # [doc = " `UnsizedPlace(p)`: `p` itself is a thin pointer (indirect place)."] # [doc = " `*p` is the wide pointer that references the actual unsized place."] # [doc = ""] # [doc = " MIR only supports unsized args, not dynamically-sized locals, so"] # [doc = " new unsized temps don't exist and we must reuse the referred-to place."] # [doc = ""] # [doc = " FIXME: Since the removal of unsized locals in <https://github.com/rust-lang/rust/pull/142911>,"] # [doc = " can we maybe use `Place` here? Or refactor it in another way? There are quite a few"] # [doc = " `UnsizedPlace => bug` branches now."] UnsizedPlace (PlaceRef < 'tcx , V >) , # [doc = " The backend [`OperandValue`] has already been generated."] Operand (OperandRef < 'tcx , V >) , # [doc = " Will be a `Self::Operand` once we get to its definition."] PendingOperand , }
    };
}

LocalRef!()