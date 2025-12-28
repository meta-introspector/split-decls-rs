macro_rules! deps {
    () => {
        NllRegionVariableOrigin!();
        BoundRegionConversionTime!();
        TypeVariableOrigin!();
    };
}

macro_rules! RegionVariableOrigin {
    () => {
        deps!();
        # [doc = " Reasons to create a region inference variable."] # [doc = ""] # [doc = " See `error_reporting` module for more details."] # [derive (Copy , Clone , Debug)] pub enum RegionVariableOrigin { # [doc = " Region variables created for ill-categorized reasons."] # [doc = ""] # [doc = " They mostly indicate places in need of refactoring."] Misc (Span) , # [doc = " Regions created by a `&P` or `[...]` pattern."] PatternRegion (Span) , # [doc = " Regions created by `&` operator."] BorrowRegion (Span) , # [doc = " Regions created as part of an autoref of a method receiver."] Autoref (Span) , # [doc = " Regions created as part of an automatic coercion."] Coercion (Span) , # [doc = " Region variables created as the values for early-bound regions."] # [doc = ""] # [doc = " FIXME(@lcnr): This should also store a `DefId`, similar to"] # [doc = " `TypeVariableOrigin`."] RegionParameterDefinition (Span , Symbol) , # [doc = " Region variables created when instantiating a binder with"] # [doc = " existential variables, e.g. when calling a function or method."] BoundRegion (Span , ty :: BoundRegionKind , BoundRegionConversionTime) , UpvarRegion (ty :: UpvarId , Span) , # [doc = " This origin is used for the inference variables that we create"] # [doc = " during NLL region processing."] Nll (NllRegionVariableOrigin) , }
    };
}

RegionVariableOrigin!();