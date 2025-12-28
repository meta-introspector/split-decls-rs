macro_rules! deps {
    () => {
        FieldResolverParameter!();
    };
}

macro_rules! FieldResolver {
    () => {
        deps!();
        # [doc = " IR representation of a field resolver."] struct FieldResolver < 'a > { resolver_fn_ident : & 'a Ident , cfg_attrs : & 'a [Attribute] , # [doc = " Parsed Parameters for this resolver"] params : & 'a [FieldResolverParameter < 'a >] , }
    };
}

FieldResolver!();