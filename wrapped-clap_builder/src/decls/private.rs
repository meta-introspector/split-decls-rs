macro_rules! deps {
    () => {
        ValueEnum!();
        ValueParserFactory!();
        OsStr!();
        Error!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { use super :: * ; # [allow (non_camel_case_types)] pub trait _impls_ValueParserFactorySealed { } impl < P : ValueParserFactory > _impls_ValueParserFactorySealed for & & & & & & _infer_ValueParser_for < P > { } # [allow (non_camel_case_types)] pub trait _impls_ValueEnumSealed { } impl < E : crate :: ValueEnum > _impls_ValueEnumSealed for & & & & & _infer_ValueParser_for < E > { } # [allow (non_camel_case_types)] pub trait _impls_From_OsStringSealed { } impl < FromOsString > _impls_From_OsStringSealed for & & & & _infer_ValueParser_for < FromOsString > where FromOsString : From < std :: ffi :: OsString > + std :: any :: Any + Send + Sync + 'static { } # [allow (non_camel_case_types)] pub trait _impls_From_OsStrSealed { } impl < FromOsStr > _impls_From_OsStrSealed for & & & _infer_ValueParser_for < FromOsStr > where FromOsStr : for < 's > From < & 's std :: ffi :: OsStr > + std :: any :: Any + Send + Sync + 'static { } # [allow (non_camel_case_types)] pub trait _impls_From_StringSealed { } impl < FromString > _impls_From_StringSealed for & & _infer_ValueParser_for < FromString > where FromString : From < String > + std :: any :: Any + Send + Sync + 'static { } # [allow (non_camel_case_types)] pub trait _impls_From_strSealed { } impl < FromStr > _impls_From_strSealed for & _infer_ValueParser_for < FromStr > where FromStr : for < 's > From < & 's str > + std :: any :: Any + Send + Sync + 'static { } # [allow (non_camel_case_types)] pub trait _impls_FromStrSealed { } impl < Parse > _impls_FromStrSealed for _infer_ValueParser_for < Parse > where Parse : std :: str :: FromStr + std :: any :: Any + Send + Sync + 'static , < Parse as std :: str :: FromStr > :: Err : Into < Box < dyn std :: error :: Error + Send + Sync + 'static > > , { } }
    };
}

private!()