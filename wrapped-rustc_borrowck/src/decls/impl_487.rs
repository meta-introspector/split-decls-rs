macro_rules! deps {
    () => {
        InstantiateOpaqueType!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl < 'tcx > TypeOp < 'tcx > for InstantiateOpaqueType < 'tcx > { type Output = () ; # [doc = " We use this type itself to store the information used"] # [doc = " when reporting errors. Since this is not a query, we don't"] # [doc = " re-run anything during error reporting - we just use the information"] # [doc = " we saved to help extract an error from the already-existing region"] # [doc = " constraints in our `InferCtxt`"] type ErrorInfo = InstantiateOpaqueType < 'tcx > ; fn fully_perform (mut self , infcx : & InferCtxt < 'tcx > , root_def_id : LocalDefId , span : Span ,) -> Result < TypeOpOutput < 'tcx , Self > , ErrorGuaranteed > { let (mut output , region_constraints) = scrape_region_constraints (infcx , root_def_id , "InstantiateOpaqueType" , span , | ocx | { ocx . register_obligations (self . obligations . clone ()) ; Ok (()) }) ? ; self . region_constraints = Some (region_constraints) ; output . error_info = Some (self) ; Ok (output) } }
    };
}

impl_487!()