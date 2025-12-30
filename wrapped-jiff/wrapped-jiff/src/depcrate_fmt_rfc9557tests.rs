// Generated macro for tests (module)
macro_rules! Depcrate_fmt_rfc9557tests {
() => {
// Module: crate::fmt::rfc9557
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn ok_time_zone () { if crate :: tz :: db () . is_definitively_empty () { return ; } let p = | input | { Parser :: new () . parse (input) . unwrap () . value . to_time_zone_annotation () . unwrap () . map (| ann | (ann . to_time_zone () . unwrap () , ann . is_critical ())) } ; insta :: assert_debug_snapshot ! (p (b"[America/New_York]") , @ r###"
        Some(
            (
                TimeZone(
                    TZif(
                        "America/New_York",
                    ),
                ),
                false,
            ),
        )
        "###) ; insta :: assert_debug_snapshot ! (p (b"[!America/New_York]") , @ r###"
        Some(
            (
                TimeZone(
                    TZif(
                        "America/New_York",
                    ),
                ),
                true,
            ),
        )
        "###) ; insta :: assert_debug_snapshot ! (p (b"[america/new_york]") , @ r###"
        Some(
            (
                TimeZone(
                    TZif(
                        "America/New_York",
                    ),
                ),
                false,
            ),
        )
        "###) ; insta :: assert_debug_snapshot ! (p (b"[+25:59]") , @ r###"
        Some(
            (
                TimeZone(
                    25:59:00,
                ),
                false,
            ),
        )
        "###) ; insta :: assert_debug_snapshot ! (p (b"[-25:59]") , @ r###"
        Some(
            (
                TimeZone(
                    -25:59:00,
                ),
                false,
            ),
        )
        "###) ; } # [test] fn ok_empty () { let p = | input | Parser :: new () . parse (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "",
                time_zone: None,
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"blah") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "",
                time_zone: None,
            },
            input: "blah",
        }
        "###) ; } # [test] fn ok_unsupported () { let p = | input | Parser :: new () . parse (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"[u-ca=chinese]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[u-ca=chinese]",
                time_zone: None,
            },
            input: "",
        }
        "### ,) ; insta :: assert_debug_snapshot ! (p (b"[u-ca=chinese-japanese]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[u-ca=chinese-japanese]",
                time_zone: None,
            },
            input: "",
        }
        "### ,) ; insta :: assert_debug_snapshot ! (p (b"[u-ca=chinese-japanese-russian]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[u-ca=chinese-japanese-russian]",
                time_zone: None,
            },
            input: "",
        }
        "### ,) ; } # [test] fn ok_iana () { let p = | input | Parser :: new () . parse (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"[America/New_York]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[America/New_York]",
                time_zone: Some(
                    Named {
                        critical: false,
                        name: "America/New_York",
                    },
                ),
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"[!America/New_York]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[!America/New_York]",
                time_zone: Some(
                    Named {
                        critical: true,
                        name: "America/New_York",
                    },
                ),
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"[UTC]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[UTC]",
                time_zone: Some(
                    Named {
                        critical: false,
                        name: "UTC",
                    },
                ),
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"[.._foo_../.0+-]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[.._foo_../.0+-]",
                time_zone: Some(
                    Named {
                        critical: false,
                        name: ".._foo_../.0+-",
                    },
                ),
            },
            input: "",
        }
        "###) ; } # [test] fn ok_offset () { let p = | input | Parser :: new () . parse (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"[-00]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[-00]",
                time_zone: Some(
                    Offset {
                        critical: false,
                        offset: ParsedOffset {
                            kind: Numeric(
                                -00,
                            ),
                        },
                    },
                ),
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"[+00]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[+00]",
                time_zone: Some(
                    Offset {
                        critical: false,
                        offset: ParsedOffset {
                            kind: Numeric(
                                +00,
                            ),
                        },
                    },
                ),
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"[-05]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[-05]",
                time_zone: Some(
                    Offset {
                        critical: false,
                        offset: ParsedOffset {
                            kind: Numeric(
                                -05,
                            ),
                        },
                    },
                ),
            },
            input: "",
        }
        "###) ; insta :: assert_debug_snapshot ! (p (b"[!+05:12]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[!+05:12]",
                time_zone: Some(
                    Offset {
                        critical: true,
                        offset: ParsedOffset {
                            kind: Numeric(
                                +05:12,
                            ),
                        },
                    },
                ),
            },
            input: "",
        }
        "###) ; } # [test] fn ok_iana_unsupported () { let p = | input | Parser :: new () . parse (input) . unwrap () ; insta :: assert_debug_snapshot ! (p (b"[America/New_York][u-ca=chinese-japanese-russian]") , @ r###"
        Parsed {
            value: ParsedAnnotations {
                input: "[America/New_York][u-ca=chinese-japanese-russian]",
                time_zone: Some(
                    Named {
                        critical: false,
                        name: "America/New_York",
                    },
                ),
            },
            input: "",
        }
        "### ,) ; } # [test] fn err_iana () { insta :: assert_snapshot ! (Parser :: new () . parse (b"[0/Foo]") . unwrap_err () , @ r###"expected ASCII alphabetic byte (or underscore or period) at the start of an RFC 9557 annotation or time zone component name, but found "0" instead"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[Foo/0Bar]") . unwrap_err () , @ r###"expected ASCII alphabetic byte (or underscore or period) at the start of an RFC 9557 annotation or time zone component name, but found "0" instead"### ,) ; } # [test] fn err_offset () { insta :: assert_snapshot ! (Parser :: new () . parse (b"[+") . unwrap_err () , @ r###"failed to parse hours in UTC numeric offset "+": expected two digit hour after sign, but found end of input"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[+26]") . unwrap_err () , @ r###"failed to parse hours in UTC numeric offset "+26]": offset hours are not valid: parameter 'hours' with value 26 is not in the required range of 0..=25"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[-26]") . unwrap_err () , @ r###"failed to parse hours in UTC numeric offset "-26]": offset hours are not valid: parameter 'hours' with value 26 is not in the required range of 0..=25"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[+05:12:34]") . unwrap_err () , @ r###"subminute precision for UTC numeric offset "+05:12:34]" is not enabled in this context (must provide only integral minutes)"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[+05:12:34.123456789]") . unwrap_err () , @ r###"subminute precision for UTC numeric offset "+05:12:34.123456789]" is not enabled in this context (must provide only integral minutes)"### ,) ; } # [test] fn err_critical_unsupported () { insta :: assert_snapshot ! (Parser :: new () . parse (b"[!u-ca=chinese]") . unwrap_err () , @ r###"found unsupported RFC 9557 annotation with key "u-ca" with the critical flag ('!') set"### ,) ; } # [test] fn err_key_leading_char () { insta :: assert_snapshot ! (Parser :: new () . parse (b"[") . unwrap_err () , @ "expected the start of an RFC 9557 annotation or IANA time zone component name, but found end of input instead" ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[&") . unwrap_err () , @ r###"expected ASCII alphabetic byte (or underscore or period) at the start of an RFC 9557 annotation or time zone component name, but found "&" instead"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[Foo][") . unwrap_err () , @ "expected the start of an RFC 9557 annotation key, but found end of input instead" ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[Foo][&") . unwrap_err () , @ r###"expected lowercase alphabetic byte (or underscore) at the start of an RFC 9557 annotation key, but found "&" instead"### ,) ; } # [test] fn err_separator () { insta :: assert_snapshot ! (Parser :: new () . parse (b"[abc") . unwrap_err () , @ "expected an ']' after parsing an RFC 9557 time zone annotation, but found end of input instead" ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[_abc") . unwrap_err () , @ "expected an ']' after parsing an RFC 9557 time zone annotation, but found end of input instead" ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[abc^") . unwrap_err () , @ r###"expected an ']' after parsing an RFC 9557 time zone annotation, but found "^" instead"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[Foo][abc") . unwrap_err () , @ "expected an '=' after parsing an RFC 9557 annotation key, but found end of input instead" ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[Foo][_abc") . unwrap_err () , @ "expected an '=' after parsing an RFC 9557 annotation key, but found end of input instead" ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[Foo][abc^") . unwrap_err () , @ r###"expected an '=' after parsing an RFC 9557 annotation key, but found "^" instead"### ,) ; } # [test] fn err_value () { insta :: assert_snapshot ! (Parser :: new () . parse (b"[abc=") . unwrap_err () , @ "expected the start of an RFC 9557 annotation value, but found end of input instead" ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[_abc=") . unwrap_err () , @ "expected the start of an RFC 9557 annotation value, but found end of input instead" ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[abc=^") . unwrap_err () , @ r###"expected alphanumeric ASCII byte at the start of an RFC 9557 annotation value, but found "^" instead"### ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[abc=]") . unwrap_err () , @ r###"expected alphanumeric ASCII byte at the start of an RFC 9557 annotation value, but found "]" instead"### ,) ; } # [test] fn err_close () { insta :: assert_snapshot ! (Parser :: new () . parse (b"[abc=123") . unwrap_err () , @ "expected an ']' after parsing an RFC 9557 annotation key and value, but found end of input instead" ,) ; insta :: assert_snapshot ! (Parser :: new () . parse (b"[abc=123*") . unwrap_err () , @ r###"expected an ']' after parsing an RFC 9557 annotation key and value, but found "*" instead"### ,) ; } # [cfg (feature = "std")] # [test] fn err_time_zone_db_lookup () { if crate :: tz :: db () . is_definitively_empty () { return ; } let p = | input | { Parser :: new () . parse (input) . unwrap () . value . to_time_zone_annotation () . unwrap () . unwrap () . to_time_zone () . unwrap_err () } ; insta :: assert_snapshot ! (p (b"[Foo]") , @ "failed to find time zone `Foo` in time zone database" ,) ; } # [test] fn err_repeated_time_zone () { let p = | input | Parser :: new () . parse (input) . unwrap_err () ; insta :: assert_snapshot ! (p (b"[america/new_york][america/new_york]") , @ "expected an '=' after parsing an RFC 9557 annotation key, but found / instead (time zone annotations must come first)" ,) ; } }
};
}
