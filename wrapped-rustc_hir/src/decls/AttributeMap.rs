macro_rules! deps {
    () => {
        Attribute!();
    };
}

macro_rules! AttributeMap {
    () => {
        deps!();
        # [doc = " Attributes owned by a HIR owner."] # [derive (Debug)] pub struct AttributeMap < 'tcx > { pub map : SortedMap < ItemLocalId , & 'tcx [Attribute] > , # [doc = " Preprocessed `#[define_opaque]` attribute."] pub define_opaque : Option < & 'tcx [(Span , LocalDefId)] > , pub opt_hash : Option < Fingerprint > , }
    };
}

AttributeMap!();