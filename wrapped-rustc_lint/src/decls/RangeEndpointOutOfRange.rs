macro_rules! deps {
    () => {
        UseInclusiveRange!();
    };
}

macro_rules! RangeEndpointOutOfRange {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_range_endpoint_out_of_range)] pub (crate) struct RangeEndpointOutOfRange < 'a > { pub ty : & 'a str , # [subdiagnostic] pub sub : UseInclusiveRange < 'a > , }
    };
}

RangeEndpointOutOfRange!();