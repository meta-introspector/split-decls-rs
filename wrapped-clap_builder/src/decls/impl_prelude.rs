macro_rules! deps {
    () => {
        ValueEnum!();
        EnumValueParser!();
        StringValueParser!();
        Error!();
        ValueParserFactory!();
        Result!();
        Parser!();
        ValueParser!();
        OsStringValueParser!();
        OsStr!();
    };
}

macro_rules! impl_prelude {
    () => {
        deps!();
        # [doc (hidden)] pub mod impl_prelude { use super :: * ; # [doc (hidden)] # [allow (non_camel_case_types)] pub trait _impls_ValueParserFactory : private :: _impls_ValueParserFactorySealed { type Parser ; fn value_parser (& self) -> Self :: Parser ; } impl < P : ValueParserFactory > _impls_ValueParserFactory for & & & & & & _infer_ValueParser_for < P > { type Parser = P :: Parser ; fn value_parser (& self) -> Self :: Parser { P :: value_parser () } } # [doc (hidden)] # [allow (non_camel_case_types)] pub trait _impls_ValueEnum : private :: _impls_ValueEnumSealed { type Output ; fn value_parser (& self) -> Self :: Output ; } impl < E : crate :: ValueEnum + Clone + Send + Sync + 'static > _impls_ValueEnum for & & & & & _infer_ValueParser_for < E > { type Output = EnumValueParser < E > ; fn value_parser (& self) -> Self :: Output { EnumValueParser :: < E > :: new () } } # [doc (hidden)] # [allow (non_camel_case_types)] pub trait _impls_From_OsString : private :: _impls_From_OsStringSealed { fn value_parser (& self) -> _AnonymousValueParser ; } impl < FromOsString > _impls_From_OsString for & & & & _infer_ValueParser_for < FromOsString > where FromOsString : From < std :: ffi :: OsString > + std :: any :: Any + Clone + Send + Sync + 'static , { fn value_parser (& self) -> _AnonymousValueParser { _AnonymousValueParser (OsStringValueParser :: new () . map (| s | FromOsString :: from (s)) . into () ,) } } # [doc (hidden)] # [allow (non_camel_case_types)] pub trait _impls_From_OsStr : private :: _impls_From_OsStrSealed { fn value_parser (& self) -> _AnonymousValueParser ; } impl < FromOsStr > _impls_From_OsStr for & & & _infer_ValueParser_for < FromOsStr > where FromOsStr : for < 's > From < & 's std :: ffi :: OsStr > + std :: any :: Any + Clone + Send + Sync + 'static , { fn value_parser (& self) -> _AnonymousValueParser { _AnonymousValueParser (OsStringValueParser :: new () . map (| s | FromOsStr :: from (& s)) . into () ,) } } # [doc (hidden)] # [allow (non_camel_case_types)] pub trait _impls_From_String : private :: _impls_From_StringSealed { fn value_parser (& self) -> _AnonymousValueParser ; } impl < FromString > _impls_From_String for & & _infer_ValueParser_for < FromString > where FromString : From < String > + std :: any :: Any + Clone + Send + Sync + 'static , { fn value_parser (& self) -> _AnonymousValueParser { _AnonymousValueParser (StringValueParser :: new () . map (| s | FromString :: from (s)) . into ()) } } # [doc (hidden)] # [allow (non_camel_case_types)] pub trait _impls_From_str : private :: _impls_From_strSealed { fn value_parser (& self) -> _AnonymousValueParser ; } impl < FromStr > _impls_From_str for & _infer_ValueParser_for < FromStr > where FromStr : for < 's > From < & 's str > + std :: any :: Any + Clone + Send + Sync + 'static , { fn value_parser (& self) -> _AnonymousValueParser { _AnonymousValueParser (StringValueParser :: new () . map (| s | FromStr :: from (& s)) . into ()) } } # [doc (hidden)] # [allow (non_camel_case_types)] pub trait _impls_FromStr : private :: _impls_FromStrSealed { fn value_parser (& self) -> _AnonymousValueParser ; } impl < Parse > _impls_FromStr for _infer_ValueParser_for < Parse > where Parse : std :: str :: FromStr + std :: any :: Any + Clone + Send + Sync + 'static , < Parse as std :: str :: FromStr > :: Err : Into < Box < dyn std :: error :: Error + Send + Sync + 'static > > , { fn value_parser (& self) -> _AnonymousValueParser { let func : fn (& str) -> Result < Parse , < Parse as std :: str :: FromStr > :: Err > = Parse :: from_str ; _AnonymousValueParser (ValueParser :: new (func)) } } }
    };
}

impl_prelude!()