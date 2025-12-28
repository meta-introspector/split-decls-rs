macro_rules! deps {
    () => {
        UnknownMetaItem!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for UnknownMetaItem < '_ > { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let expected = self . expected . iter () . map (| name | format ! ("`{name}`")) . collect :: < Vec < _ > > () ; Diag :: new (dcx , level , fluent :: attr_parsing_unknown_meta_item) . with_span (self . span) . with_code (E0541) . with_arg ("item" , self . item) . with_arg ("expected" , expected . join (", ")) . with_span_label (self . span , fluent :: attr_parsing_label) } }
    };
}

impl_318!()