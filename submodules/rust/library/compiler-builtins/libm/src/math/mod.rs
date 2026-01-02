mkitem!{macro_rules ! force_eval { ($ e : expr) => { unsafe { :: core :: ptr :: read_volatile (&$ e) } } ; }}
mkitem!{# [cfg (not (debug_assertions))] macro_rules ! i { ($ array : expr , $ index : expr) => { unsafe { *$ array . get_unchecked ($ index) } } ; ($ array : expr , $ index : expr , = , $ rhs : expr) => { unsafe { *$ array . get_unchecked_mut ($ index) = $ rhs ; } } ; ($ array : expr , $ index : expr , += , $ rhs : expr) => { unsafe { *$ array . get_unchecked_mut ($ index) += $ rhs ; } } ; ($ array : expr , $ index : expr , -= , $ rhs : expr) => { unsafe { *$ array . get_unchecked_mut ($ index) -= $ rhs ; } } ; ($ array : expr , $ index : expr , &= , $ rhs : expr) => { unsafe { *$ array . get_unchecked_mut ($ index) &= $ rhs ; } } ; ($ array : expr , $ index : expr , == , $ rhs : expr) => { unsafe { *$ array . get_unchecked_mut ($ index) == $ rhs } } ; }}
mkitem!{# [cfg (debug_assertions)] macro_rules ! i { ($ array : expr , $ index : expr) => { *$ array . get ($ index) . unwrap () } ; ($ array : expr , $ index : expr , = , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () = $ rhs ; } ; ($ array : expr , $ index : expr , -= , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () -= $ rhs ; } ; ($ array : expr , $ index : expr , += , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () += $ rhs ; } ; ($ array : expr , $ index : expr , &= , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () &= $ rhs ; } ; ($ array : expr , $ index : expr , == , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () == $ rhs } ; }}
mkitem!{# [cfg (any (debug_assertions , not (intrinsics_enabled)))] macro_rules ! div { ($ a : expr , $ b : expr) => { $ a / $ b } ; }}
mkitem!{# [cfg (all (not (debug_assertions) , intrinsics_enabled))] macro_rules ! div { ($ a : expr , $ b : expr) => { unsafe { core :: intrinsics :: unchecked_div ($ a , $ b) } } ; }}
mkmod!{support, { 
                getname!(support);
                getsrc!(support);
                getpath!(support);
                get_deps!(support);
                get_crates!(support);
                mkinclude!(support);
                 
            }}
mkmod!{support, { 
                getname!(support);
                getsrc!(support);
                getpath!(support);
                get_deps!(support);
                get_crates!(support);
                mkinclude!(support);
                 
            }}
mkitem!{cfg_if ! { if # [cfg (feature = "unstable-public-internals")] { pub mod generic ; } else { mod generic ; } }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                 
            }}
mkmod!{expo2, { 
                getname!(expo2);
                getsrc!(expo2);
                getpath!(expo2);
                get_deps!(expo2);
                get_crates!(expo2);
                mkinclude!(expo2);
                 
            }}
mkmod!{k_cos, { 
                getname!(k_cos);
                getsrc!(k_cos);
                getpath!(k_cos);
                get_deps!(k_cos);
                get_crates!(k_cos);
                mkinclude!(k_cos);
                 
            }}
mkmod!{k_cosf, { 
                getname!(k_cosf);
                getsrc!(k_cosf);
                getpath!(k_cosf);
                get_deps!(k_cosf);
                get_crates!(k_cosf);
                mkinclude!(k_cosf);
                 
            }}
mkmod!{k_expo2, { 
                getname!(k_expo2);
                getsrc!(k_expo2);
                getpath!(k_expo2);
                get_deps!(k_expo2);
                get_crates!(k_expo2);
                mkinclude!(k_expo2);
                 
            }}
mkmod!{k_expo2f, { 
                getname!(k_expo2f);
                getsrc!(k_expo2f);
                getpath!(k_expo2f);
                get_deps!(k_expo2f);
                get_crates!(k_expo2f);
                mkinclude!(k_expo2f);
                 
            }}
mkmod!{k_sin, { 
                getname!(k_sin);
                getsrc!(k_sin);
                getpath!(k_sin);
                get_deps!(k_sin);
                get_crates!(k_sin);
                mkinclude!(k_sin);
                 
            }}
mkmod!{k_sinf, { 
                getname!(k_sinf);
                getsrc!(k_sinf);
                getpath!(k_sinf);
                get_deps!(k_sinf);
                get_crates!(k_sinf);
                mkinclude!(k_sinf);
                 
            }}
mkmod!{k_tan, { 
                getname!(k_tan);
                getsrc!(k_tan);
                getpath!(k_tan);
                get_deps!(k_tan);
                get_crates!(k_tan);
                mkinclude!(k_tan);
                 
            }}
mkmod!{k_tanf, { 
                getname!(k_tanf);
                getsrc!(k_tanf);
                getpath!(k_tanf);
                get_deps!(k_tanf);
                get_crates!(k_tanf);
                mkinclude!(k_tanf);
                 
            }}
mkmod!{rem_pio2, { 
                getname!(rem_pio2);
                getsrc!(rem_pio2);
                getpath!(rem_pio2);
                get_deps!(rem_pio2);
                get_crates!(rem_pio2);
                mkinclude!(rem_pio2);
                 
            }}
mkmod!{rem_pio2_large, { 
                getname!(rem_pio2_large);
                getsrc!(rem_pio2_large);
                getpath!(rem_pio2_large);
                get_deps!(rem_pio2_large);
                get_crates!(rem_pio2_large);
                mkinclude!(rem_pio2_large);
                 
            }}
mkmod!{rem_pio2f, { 
                getname!(rem_pio2f);
                getsrc!(rem_pio2f);
                getpath!(rem_pio2f);
                get_deps!(rem_pio2f);
                get_crates!(rem_pio2f);
                mkinclude!(rem_pio2f);
                 
            }}
mkuse!{use self :: expo2 :: expo2 ;}
mkuse!{use self :: k_cos :: k_cos ;}
mkuse!{use self :: k_cosf :: k_cosf ;}
mkuse!{use self :: k_expo2 :: k_expo2 ;}
mkuse!{use self :: k_expo2f :: k_expo2f ;}
mkuse!{use self :: k_sin :: k_sin ;}
mkuse!{use self :: k_sinf :: k_sinf ;}
mkuse!{use self :: k_tan :: k_tan ;}
mkuse!{use self :: k_tanf :: k_tanf ;}
mkuse!{use self :: rem_pio2 :: rem_pio2 ;}
mkuse!{use self :: rem_pio2_large :: rem_pio2_large ;}
mkuse!{use self :: rem_pio2f :: rem_pio2f ;}
mkuse!{# [allow (unused_imports)] use self :: support :: { CastFrom , CastInto , DFloat , DInt , Float , HFloat , HInt , Int , IntTy , MinInt } ;}
mkmod!{acos, { 
                getname!(acos);
                getsrc!(acos);
                getpath!(acos);
                get_deps!(acos);
                get_crates!(acos);
                mkinclude!(acos);
                 
            }}
mkmod!{acosf, { 
                getname!(acosf);
                getsrc!(acosf);
                getpath!(acosf);
                get_deps!(acosf);
                get_crates!(acosf);
                mkinclude!(acosf);
                 
            }}
mkmod!{acosh, { 
                getname!(acosh);
                getsrc!(acosh);
                getpath!(acosh);
                get_deps!(acosh);
                get_crates!(acosh);
                mkinclude!(acosh);
                 
            }}
mkmod!{acoshf, { 
                getname!(acoshf);
                getsrc!(acoshf);
                getpath!(acoshf);
                get_deps!(acoshf);
                get_crates!(acoshf);
                mkinclude!(acoshf);
                 
            }}
mkmod!{asin, { 
                getname!(asin);
                getsrc!(asin);
                getpath!(asin);
                get_deps!(asin);
                get_crates!(asin);
                mkinclude!(asin);
                 
            }}
mkmod!{asinf, { 
                getname!(asinf);
                getsrc!(asinf);
                getpath!(asinf);
                get_deps!(asinf);
                get_crates!(asinf);
                mkinclude!(asinf);
                 
            }}
mkmod!{asinh, { 
                getname!(asinh);
                getsrc!(asinh);
                getpath!(asinh);
                get_deps!(asinh);
                get_crates!(asinh);
                mkinclude!(asinh);
                 
            }}
mkmod!{asinhf, { 
                getname!(asinhf);
                getsrc!(asinhf);
                getpath!(asinhf);
                get_deps!(asinhf);
                get_crates!(asinhf);
                mkinclude!(asinhf);
                 
            }}
mkmod!{atan, { 
                getname!(atan);
                getsrc!(atan);
                getpath!(atan);
                get_deps!(atan);
                get_crates!(atan);
                mkinclude!(atan);
                 
            }}
mkmod!{atan2, { 
                getname!(atan2);
                getsrc!(atan2);
                getpath!(atan2);
                get_deps!(atan2);
                get_crates!(atan2);
                mkinclude!(atan2);
                 
            }}
mkmod!{atan2f, { 
                getname!(atan2f);
                getsrc!(atan2f);
                getpath!(atan2f);
                get_deps!(atan2f);
                get_crates!(atan2f);
                mkinclude!(atan2f);
                 
            }}
mkmod!{atanf, { 
                getname!(atanf);
                getsrc!(atanf);
                getpath!(atanf);
                get_deps!(atanf);
                get_crates!(atanf);
                mkinclude!(atanf);
                 
            }}
mkmod!{atanh, { 
                getname!(atanh);
                getsrc!(atanh);
                getpath!(atanh);
                get_deps!(atanh);
                get_crates!(atanh);
                mkinclude!(atanh);
                 
            }}
mkmod!{atanhf, { 
                getname!(atanhf);
                getsrc!(atanhf);
                getpath!(atanhf);
                get_deps!(atanhf);
                get_crates!(atanhf);
                mkinclude!(atanhf);
                 
            }}
mkmod!{cbrt, { 
                getname!(cbrt);
                getsrc!(cbrt);
                getpath!(cbrt);
                get_deps!(cbrt);
                get_crates!(cbrt);
                mkinclude!(cbrt);
                 
            }}
mkmod!{cbrtf, { 
                getname!(cbrtf);
                getsrc!(cbrtf);
                getpath!(cbrtf);
                get_deps!(cbrtf);
                get_crates!(cbrtf);
                mkinclude!(cbrtf);
                 
            }}
mkmod!{ceil, { 
                getname!(ceil);
                getsrc!(ceil);
                getpath!(ceil);
                get_deps!(ceil);
                get_crates!(ceil);
                mkinclude!(ceil);
                 
            }}
mkmod!{copysign, { 
                getname!(copysign);
                getsrc!(copysign);
                getpath!(copysign);
                get_deps!(copysign);
                get_crates!(copysign);
                mkinclude!(copysign);
                 
            }}
mkmod!{cos, { 
                getname!(cos);
                getsrc!(cos);
                getpath!(cos);
                get_deps!(cos);
                get_crates!(cos);
                mkinclude!(cos);
                 
            }}
mkmod!{cosf, { 
                getname!(cosf);
                getsrc!(cosf);
                getpath!(cosf);
                get_deps!(cosf);
                get_crates!(cosf);
                mkinclude!(cosf);
                 
            }}
mkmod!{cosh, { 
                getname!(cosh);
                getsrc!(cosh);
                getpath!(cosh);
                get_deps!(cosh);
                get_crates!(cosh);
                mkinclude!(cosh);
                 
            }}
mkmod!{coshf, { 
                getname!(coshf);
                getsrc!(coshf);
                getpath!(coshf);
                get_deps!(coshf);
                get_crates!(coshf);
                mkinclude!(coshf);
                 
            }}
mkmod!{erf, { 
                getname!(erf);
                getsrc!(erf);
                getpath!(erf);
                get_deps!(erf);
                get_crates!(erf);
                mkinclude!(erf);
                 
            }}
mkmod!{erff, { 
                getname!(erff);
                getsrc!(erff);
                getpath!(erff);
                get_deps!(erff);
                get_crates!(erff);
                mkinclude!(erff);
                 
            }}
mkmod!{exp, { 
                getname!(exp);
                getsrc!(exp);
                getpath!(exp);
                get_deps!(exp);
                get_crates!(exp);
                mkinclude!(exp);
                 
            }}
mkmod!{exp10, { 
                getname!(exp10);
                getsrc!(exp10);
                getpath!(exp10);
                get_deps!(exp10);
                get_crates!(exp10);
                mkinclude!(exp10);
                 
            }}
mkmod!{exp10f, { 
                getname!(exp10f);
                getsrc!(exp10f);
                getpath!(exp10f);
                get_deps!(exp10f);
                get_crates!(exp10f);
                mkinclude!(exp10f);
                 
            }}
mkmod!{exp2, { 
                getname!(exp2);
                getsrc!(exp2);
                getpath!(exp2);
                get_deps!(exp2);
                get_crates!(exp2);
                mkinclude!(exp2);
                 
            }}
mkmod!{exp2f, { 
                getname!(exp2f);
                getsrc!(exp2f);
                getpath!(exp2f);
                get_deps!(exp2f);
                get_crates!(exp2f);
                mkinclude!(exp2f);
                 
            }}
mkmod!{expf, { 
                getname!(expf);
                getsrc!(expf);
                getpath!(expf);
                get_deps!(expf);
                get_crates!(expf);
                mkinclude!(expf);
                 
            }}
mkmod!{expm1, { 
                getname!(expm1);
                getsrc!(expm1);
                getpath!(expm1);
                get_deps!(expm1);
                get_crates!(expm1);
                mkinclude!(expm1);
                 
            }}
mkmod!{expm1f, { 
                getname!(expm1f);
                getsrc!(expm1f);
                getpath!(expm1f);
                get_deps!(expm1f);
                get_crates!(expm1f);
                mkinclude!(expm1f);
                 
            }}
mkmod!{fabs, { 
                getname!(fabs);
                getsrc!(fabs);
                getpath!(fabs);
                get_deps!(fabs);
                get_crates!(fabs);
                mkinclude!(fabs);
                 
            }}
mkmod!{fdim, { 
                getname!(fdim);
                getsrc!(fdim);
                getpath!(fdim);
                get_deps!(fdim);
                get_crates!(fdim);
                mkinclude!(fdim);
                 
            }}
mkmod!{floor, { 
                getname!(floor);
                getsrc!(floor);
                getpath!(floor);
                get_deps!(floor);
                get_crates!(floor);
                mkinclude!(floor);
                 
            }}
mkmod!{fma, { 
                getname!(fma);
                getsrc!(fma);
                getpath!(fma);
                get_deps!(fma);
                get_crates!(fma);
                mkinclude!(fma);
                 
            }}
mkmod!{fmin_fmax, { 
                getname!(fmin_fmax);
                getsrc!(fmin_fmax);
                getpath!(fmin_fmax);
                get_deps!(fmin_fmax);
                get_crates!(fmin_fmax);
                mkinclude!(fmin_fmax);
                 
            }}
mkmod!{fminimum_fmaximum, { 
                getname!(fminimum_fmaximum);
                getsrc!(fminimum_fmaximum);
                getpath!(fminimum_fmaximum);
                get_deps!(fminimum_fmaximum);
                get_crates!(fminimum_fmaximum);
                mkinclude!(fminimum_fmaximum);
                 
            }}
mkmod!{fminimum_fmaximum_num, { 
                getname!(fminimum_fmaximum_num);
                getsrc!(fminimum_fmaximum_num);
                getpath!(fminimum_fmaximum_num);
                get_deps!(fminimum_fmaximum_num);
                get_crates!(fminimum_fmaximum_num);
                mkinclude!(fminimum_fmaximum_num);
                 
            }}
mkmod!{fmod, { 
                getname!(fmod);
                getsrc!(fmod);
                getpath!(fmod);
                get_deps!(fmod);
                get_crates!(fmod);
                mkinclude!(fmod);
                 
            }}
mkmod!{frexp, { 
                getname!(frexp);
                getsrc!(frexp);
                getpath!(frexp);
                get_deps!(frexp);
                get_crates!(frexp);
                mkinclude!(frexp);
                 
            }}
mkmod!{frexpf, { 
                getname!(frexpf);
                getsrc!(frexpf);
                getpath!(frexpf);
                get_deps!(frexpf);
                get_crates!(frexpf);
                mkinclude!(frexpf);
                 
            }}
mkmod!{hypot, { 
                getname!(hypot);
                getsrc!(hypot);
                getpath!(hypot);
                get_deps!(hypot);
                get_crates!(hypot);
                mkinclude!(hypot);
                 
            }}
mkmod!{hypotf, { 
                getname!(hypotf);
                getsrc!(hypotf);
                getpath!(hypotf);
                get_deps!(hypotf);
                get_crates!(hypotf);
                mkinclude!(hypotf);
                 
            }}
mkmod!{ilogb, { 
                getname!(ilogb);
                getsrc!(ilogb);
                getpath!(ilogb);
                get_deps!(ilogb);
                get_crates!(ilogb);
                mkinclude!(ilogb);
                 
            }}
mkmod!{ilogbf, { 
                getname!(ilogbf);
                getsrc!(ilogbf);
                getpath!(ilogbf);
                get_deps!(ilogbf);
                get_crates!(ilogbf);
                mkinclude!(ilogbf);
                 
            }}
mkmod!{j0, { 
                getname!(j0);
                getsrc!(j0);
                getpath!(j0);
                get_deps!(j0);
                get_crates!(j0);
                mkinclude!(j0);
                 
            }}
mkmod!{j0f, { 
                getname!(j0f);
                getsrc!(j0f);
                getpath!(j0f);
                get_deps!(j0f);
                get_crates!(j0f);
                mkinclude!(j0f);
                 
            }}
mkmod!{j1, { 
                getname!(j1);
                getsrc!(j1);
                getpath!(j1);
                get_deps!(j1);
                get_crates!(j1);
                mkinclude!(j1);
                 
            }}
mkmod!{j1f, { 
                getname!(j1f);
                getsrc!(j1f);
                getpath!(j1f);
                get_deps!(j1f);
                get_crates!(j1f);
                mkinclude!(j1f);
                 
            }}
mkmod!{jn, { 
                getname!(jn);
                getsrc!(jn);
                getpath!(jn);
                get_deps!(jn);
                get_crates!(jn);
                mkinclude!(jn);
                 
            }}
mkmod!{jnf, { 
                getname!(jnf);
                getsrc!(jnf);
                getpath!(jnf);
                get_deps!(jnf);
                get_crates!(jnf);
                mkinclude!(jnf);
                 
            }}
mkmod!{ldexp, { 
                getname!(ldexp);
                getsrc!(ldexp);
                getpath!(ldexp);
                get_deps!(ldexp);
                get_crates!(ldexp);
                mkinclude!(ldexp);
                 
            }}
mkmod!{lgamma, { 
                getname!(lgamma);
                getsrc!(lgamma);
                getpath!(lgamma);
                get_deps!(lgamma);
                get_crates!(lgamma);
                mkinclude!(lgamma);
                 
            }}
mkmod!{lgamma_r, { 
                getname!(lgamma_r);
                getsrc!(lgamma_r);
                getpath!(lgamma_r);
                get_deps!(lgamma_r);
                get_crates!(lgamma_r);
                mkinclude!(lgamma_r);
                 
            }}
mkmod!{lgammaf, { 
                getname!(lgammaf);
                getsrc!(lgammaf);
                getpath!(lgammaf);
                get_deps!(lgammaf);
                get_crates!(lgammaf);
                mkinclude!(lgammaf);
                 
            }}
mkmod!{lgammaf_r, { 
                getname!(lgammaf_r);
                getsrc!(lgammaf_r);
                getpath!(lgammaf_r);
                get_deps!(lgammaf_r);
                get_crates!(lgammaf_r);
                mkinclude!(lgammaf_r);
                 
            }}
mkmod!{log, { 
                getname!(log);
                getsrc!(log);
                getpath!(log);
                get_deps!(log);
                get_crates!(log);
                mkinclude!(log);
                 
            }}
mkmod!{log10, { 
                getname!(log10);
                getsrc!(log10);
                getpath!(log10);
                get_deps!(log10);
                get_crates!(log10);
                mkinclude!(log10);
                 
            }}
mkmod!{log10f, { 
                getname!(log10f);
                getsrc!(log10f);
                getpath!(log10f);
                get_deps!(log10f);
                get_crates!(log10f);
                mkinclude!(log10f);
                 
            }}
mkmod!{log1p, { 
                getname!(log1p);
                getsrc!(log1p);
                getpath!(log1p);
                get_deps!(log1p);
                get_crates!(log1p);
                mkinclude!(log1p);
                 
            }}
mkmod!{log1pf, { 
                getname!(log1pf);
                getsrc!(log1pf);
                getpath!(log1pf);
                get_deps!(log1pf);
                get_crates!(log1pf);
                mkinclude!(log1pf);
                 
            }}
mkmod!{log2, { 
                getname!(log2);
                getsrc!(log2);
                getpath!(log2);
                get_deps!(log2);
                get_crates!(log2);
                mkinclude!(log2);
                 
            }}
mkmod!{log2f, { 
                getname!(log2f);
                getsrc!(log2f);
                getpath!(log2f);
                get_deps!(log2f);
                get_crates!(log2f);
                mkinclude!(log2f);
                 
            }}
mkmod!{logf, { 
                getname!(logf);
                getsrc!(logf);
                getpath!(logf);
                get_deps!(logf);
                get_crates!(logf);
                mkinclude!(logf);
                 
            }}
mkmod!{modf, { 
                getname!(modf);
                getsrc!(modf);
                getpath!(modf);
                get_deps!(modf);
                get_crates!(modf);
                mkinclude!(modf);
                 
            }}
mkmod!{modff, { 
                getname!(modff);
                getsrc!(modff);
                getpath!(modff);
                get_deps!(modff);
                get_crates!(modff);
                mkinclude!(modff);
                 
            }}
mkmod!{nextafter, { 
                getname!(nextafter);
                getsrc!(nextafter);
                getpath!(nextafter);
                get_deps!(nextafter);
                get_crates!(nextafter);
                mkinclude!(nextafter);
                 
            }}
mkmod!{nextafterf, { 
                getname!(nextafterf);
                getsrc!(nextafterf);
                getpath!(nextafterf);
                get_deps!(nextafterf);
                get_crates!(nextafterf);
                mkinclude!(nextafterf);
                 
            }}
mkmod!{pow, { 
                getname!(pow);
                getsrc!(pow);
                getpath!(pow);
                get_deps!(pow);
                get_crates!(pow);
                mkinclude!(pow);
                 
            }}
mkmod!{powf, { 
                getname!(powf);
                getsrc!(powf);
                getpath!(powf);
                get_deps!(powf);
                get_crates!(powf);
                mkinclude!(powf);
                 
            }}
mkmod!{remainder, { 
                getname!(remainder);
                getsrc!(remainder);
                getpath!(remainder);
                get_deps!(remainder);
                get_crates!(remainder);
                mkinclude!(remainder);
                 
            }}
mkmod!{remainderf, { 
                getname!(remainderf);
                getsrc!(remainderf);
                getpath!(remainderf);
                get_deps!(remainderf);
                get_crates!(remainderf);
                mkinclude!(remainderf);
                 
            }}
mkmod!{remquo, { 
                getname!(remquo);
                getsrc!(remquo);
                getpath!(remquo);
                get_deps!(remquo);
                get_crates!(remquo);
                mkinclude!(remquo);
                 
            }}
mkmod!{remquof, { 
                getname!(remquof);
                getsrc!(remquof);
                getpath!(remquof);
                get_deps!(remquof);
                get_crates!(remquof);
                mkinclude!(remquof);
                 
            }}
mkmod!{rint, { 
                getname!(rint);
                getsrc!(rint);
                getpath!(rint);
                get_deps!(rint);
                get_crates!(rint);
                mkinclude!(rint);
                 
            }}
mkmod!{round, { 
                getname!(round);
                getsrc!(round);
                getpath!(round);
                get_deps!(round);
                get_crates!(round);
                mkinclude!(round);
                 
            }}
mkmod!{roundeven, { 
                getname!(roundeven);
                getsrc!(roundeven);
                getpath!(roundeven);
                get_deps!(roundeven);
                get_crates!(roundeven);
                mkinclude!(roundeven);
                 
            }}
mkmod!{scalbn, { 
                getname!(scalbn);
                getsrc!(scalbn);
                getpath!(scalbn);
                get_deps!(scalbn);
                get_crates!(scalbn);
                mkinclude!(scalbn);
                 
            }}
mkmod!{sin, { 
                getname!(sin);
                getsrc!(sin);
                getpath!(sin);
                get_deps!(sin);
                get_crates!(sin);
                mkinclude!(sin);
                 
            }}
mkmod!{sincos, { 
                getname!(sincos);
                getsrc!(sincos);
                getpath!(sincos);
                get_deps!(sincos);
                get_crates!(sincos);
                mkinclude!(sincos);
                 
            }}
mkmod!{sincosf, { 
                getname!(sincosf);
                getsrc!(sincosf);
                getpath!(sincosf);
                get_deps!(sincosf);
                get_crates!(sincosf);
                mkinclude!(sincosf);
                 
            }}
mkmod!{sinf, { 
                getname!(sinf);
                getsrc!(sinf);
                getpath!(sinf);
                get_deps!(sinf);
                get_crates!(sinf);
                mkinclude!(sinf);
                 
            }}
mkmod!{sinh, { 
                getname!(sinh);
                getsrc!(sinh);
                getpath!(sinh);
                get_deps!(sinh);
                get_crates!(sinh);
                mkinclude!(sinh);
                 
            }}
mkmod!{sinhf, { 
                getname!(sinhf);
                getsrc!(sinhf);
                getpath!(sinhf);
                get_deps!(sinhf);
                get_crates!(sinhf);
                mkinclude!(sinhf);
                 
            }}
mkmod!{sqrt, { 
                getname!(sqrt);
                getsrc!(sqrt);
                getpath!(sqrt);
                get_deps!(sqrt);
                get_crates!(sqrt);
                mkinclude!(sqrt);
                 
            }}
mkmod!{tan, { 
                getname!(tan);
                getsrc!(tan);
                getpath!(tan);
                get_deps!(tan);
                get_crates!(tan);
                mkinclude!(tan);
                 
            }}
mkmod!{tanf, { 
                getname!(tanf);
                getsrc!(tanf);
                getpath!(tanf);
                get_deps!(tanf);
                get_crates!(tanf);
                mkinclude!(tanf);
                 
            }}
mkmod!{tanh, { 
                getname!(tanh);
                getsrc!(tanh);
                getpath!(tanh);
                get_deps!(tanh);
                get_crates!(tanh);
                mkinclude!(tanh);
                 
            }}
mkmod!{tanhf, { 
                getname!(tanhf);
                getsrc!(tanhf);
                getpath!(tanhf);
                get_deps!(tanhf);
                get_crates!(tanhf);
                mkinclude!(tanhf);
                 
            }}
mkmod!{tgamma, { 
                getname!(tgamma);
                getsrc!(tgamma);
                getpath!(tgamma);
                get_deps!(tgamma);
                get_crates!(tgamma);
                mkinclude!(tgamma);
                 
            }}
mkmod!{tgammaf, { 
                getname!(tgammaf);
                getsrc!(tgammaf);
                getpath!(tgammaf);
                get_deps!(tgammaf);
                get_crates!(tgammaf);
                mkinclude!(tgammaf);
                 
            }}
mkmod!{trunc, { 
                getname!(trunc);
                getsrc!(trunc);
                getpath!(trunc);
                get_deps!(trunc);
                get_crates!(trunc);
                mkinclude!(trunc);
                 
            }}
mkuse!{pub use self :: acos :: acos ;}
mkuse!{pub use self :: acosf :: acosf ;}
mkuse!{pub use self :: acosh :: acosh ;}
mkuse!{pub use self :: acoshf :: acoshf ;}
mkuse!{pub use self :: asin :: asin ;}
mkuse!{pub use self :: asinf :: asinf ;}
mkuse!{pub use self :: asinh :: asinh ;}
mkuse!{pub use self :: asinhf :: asinhf ;}
mkuse!{pub use self :: atan :: atan ;}
mkuse!{pub use self :: atan2 :: atan2 ;}
mkuse!{pub use self :: atan2f :: atan2f ;}
mkuse!{pub use self :: atanf :: atanf ;}
mkuse!{pub use self :: atanh :: atanh ;}
mkuse!{pub use self :: atanhf :: atanhf ;}
mkuse!{pub use self :: cbrt :: cbrt ;}
mkuse!{pub use self :: cbrtf :: cbrtf ;}
mkuse!{pub use self :: ceil :: { ceil , ceilf } ;}
mkuse!{pub use self :: copysign :: { copysign , copysignf } ;}
mkuse!{pub use self :: cos :: cos ;}
mkuse!{pub use self :: cosf :: cosf ;}
mkuse!{pub use self :: cosh :: cosh ;}
mkuse!{pub use self :: coshf :: coshf ;}
mkuse!{pub use self :: erf :: { erf , erfc } ;}
mkuse!{pub use self :: erff :: { erfcf , erff } ;}
mkuse!{pub use self :: exp :: exp ;}
mkuse!{pub use self :: exp2 :: exp2 ;}
mkuse!{pub use self :: exp2f :: exp2f ;}
mkuse!{pub use self :: exp10 :: exp10 ;}
mkuse!{pub use self :: exp10f :: exp10f ;}
mkuse!{pub use self :: expf :: expf ;}
mkuse!{pub use self :: expm1 :: expm1 ;}
mkuse!{pub use self :: expm1f :: expm1f ;}
mkuse!{pub use self :: fabs :: { fabs , fabsf } ;}
mkuse!{pub use self :: fdim :: { fdim , fdimf } ;}
mkuse!{pub use self :: floor :: { floor , floorf } ;}
mkuse!{pub use self :: fma :: { fma , fmaf } ;}
mkuse!{pub use self :: fmin_fmax :: { fmax , fmaxf , fmin , fminf } ;}
mkuse!{pub use self :: fminimum_fmaximum :: { fmaximum , fmaximumf , fminimum , fminimumf } ;}
mkuse!{pub use self :: fminimum_fmaximum_num :: { fmaximum_num , fmaximum_numf , fminimum_num , fminimum_numf } ;}
mkuse!{pub use self :: fmod :: { fmod , fmodf } ;}
mkuse!{pub use self :: frexp :: frexp ;}
mkuse!{pub use self :: frexpf :: frexpf ;}
mkuse!{pub use self :: hypot :: hypot ;}
mkuse!{pub use self :: hypotf :: hypotf ;}
mkuse!{pub use self :: ilogb :: ilogb ;}
mkuse!{pub use self :: ilogbf :: ilogbf ;}
mkuse!{pub use self :: j0 :: { j0 , y0 } ;}
mkuse!{pub use self :: j0f :: { j0f , y0f } ;}
mkuse!{pub use self :: j1 :: { j1 , y1 } ;}
mkuse!{pub use self :: j1f :: { j1f , y1f } ;}
mkuse!{pub use self :: jn :: { jn , yn } ;}
mkuse!{pub use self :: jnf :: { jnf , ynf } ;}
mkuse!{pub use self :: ldexp :: { ldexp , ldexpf } ;}
mkuse!{pub use self :: lgamma :: lgamma ;}
mkuse!{pub use self :: lgamma_r :: lgamma_r ;}
mkuse!{pub use self :: lgammaf :: lgammaf ;}
mkuse!{pub use self :: lgammaf_r :: lgammaf_r ;}
mkuse!{pub use self :: log :: log ;}
mkuse!{pub use self :: log1p :: log1p ;}
mkuse!{pub use self :: log1pf :: log1pf ;}
mkuse!{pub use self :: log2 :: log2 ;}
mkuse!{pub use self :: log2f :: log2f ;}
mkuse!{pub use self :: log10 :: log10 ;}
mkuse!{pub use self :: log10f :: log10f ;}
mkuse!{pub use self :: logf :: logf ;}
mkuse!{pub use self :: modf :: modf ;}
mkuse!{pub use self :: modff :: modff ;}
mkuse!{pub use self :: nextafter :: nextafter ;}
mkuse!{pub use self :: nextafterf :: nextafterf ;}
mkuse!{pub use self :: pow :: pow ;}
mkuse!{pub use self :: powf :: powf ;}
mkuse!{pub use self :: remainder :: remainder ;}
mkuse!{pub use self :: remainderf :: remainderf ;}
mkuse!{pub use self :: remquo :: remquo ;}
mkuse!{pub use self :: remquof :: remquof ;}
mkuse!{pub use self :: rint :: { rint , rintf } ;}
mkuse!{pub use self :: round :: { round , roundf } ;}
mkuse!{pub use self :: roundeven :: { roundeven , roundevenf } ;}
mkuse!{pub use self :: scalbn :: { scalbn , scalbnf } ;}
mkuse!{pub use self :: sin :: sin ;}
mkuse!{pub use self :: sincos :: sincos ;}
mkuse!{pub use self :: sincosf :: sincosf ;}
mkuse!{pub use self :: sinf :: sinf ;}
mkuse!{pub use self :: sinh :: sinh ;}
mkuse!{pub use self :: sinhf :: sinhf ;}
mkuse!{pub use self :: sqrt :: { sqrt , sqrtf } ;}
mkuse!{pub use self :: tan :: tan ;}
mkuse!{pub use self :: tanf :: tanf ;}
mkuse!{pub use self :: tanh :: tanh ;}
mkuse!{pub use self :: tanhf :: tanhf ;}
mkuse!{pub use self :: tgamma :: tgamma ;}
mkuse!{pub use self :: tgammaf :: tgammaf ;}
mkuse!{pub use self :: trunc :: { trunc , truncf } ;}
mkitem!{cfg_if ! { if # [cfg (f16_enabled)] { pub use self :: ceil :: ceilf16 ; pub use self :: copysign :: copysignf16 ; pub use self :: fabs :: fabsf16 ; pub use self :: fdim :: fdimf16 ; pub use self :: floor :: floorf16 ; pub use self :: fmin_fmax :: { fmaxf16 , fminf16 } ; pub use self :: fminimum_fmaximum :: { fmaximumf16 , fminimumf16 } ; pub use self :: fminimum_fmaximum_num :: { fmaximum_numf16 , fminimum_numf16 } ; pub use self :: fmod :: fmodf16 ; pub use self :: ldexp :: ldexpf16 ; pub use self :: rint :: rintf16 ; pub use self :: round :: roundf16 ; pub use self :: roundeven :: roundevenf16 ; pub use self :: scalbn :: scalbnf16 ; pub use self :: sqrt :: sqrtf16 ; pub use self :: trunc :: truncf16 ; # [allow (unused_imports)] pub (crate) use self :: fma :: fmaf16 ; } }}
mkitem!{cfg_if ! { if # [cfg (f128_enabled)] { pub use self :: ceil :: ceilf128 ; pub use self :: copysign :: copysignf128 ; pub use self :: fabs :: fabsf128 ; pub use self :: fdim :: fdimf128 ; pub use self :: floor :: floorf128 ; pub use self :: fma :: fmaf128 ; pub use self :: fmin_fmax :: { fmaxf128 , fminf128 } ; pub use self :: fminimum_fmaximum :: { fmaximumf128 , fminimumf128 } ; pub use self :: fminimum_fmaximum_num :: { fmaximum_numf128 , fminimum_numf128 } ; pub use self :: fmod :: fmodf128 ; pub use self :: ldexp :: ldexpf128 ; pub use self :: rint :: rintf128 ; pub use self :: round :: roundf128 ; pub use self :: roundeven :: roundevenf128 ; pub use self :: scalbn :: scalbnf128 ; pub use self :: sqrt :: sqrtf128 ; pub use self :: trunc :: truncf128 ; } }}

macro_rules! get_high_word_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_high_word in module {}", module_path!());
    };
}

mkfn!{
    get_high_word_introspect!();
    # [inline] fn get_high_word (x : f64) -> u32 { (x . to_bits () >> 32) as u32 }
}

macro_rules! get_low_word_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_low_word in module {}", module_path!());
    };
}

mkfn!{
    get_low_word_introspect!();
    # [inline] fn get_low_word (x : f64) -> u32 { x . to_bits () as u32 }
}

macro_rules! with_set_high_word_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_set_high_word in module {}", module_path!());
    };
}

mkfn!{
    with_set_high_word_introspect!();
    # [inline] fn with_set_high_word (f : f64 , hi : u32) -> f64 { let mut tmp = f . to_bits () ; tmp &= 0x00000000_ffffffff ; tmp |= (hi as u64) << 32 ; f64 :: from_bits (tmp) }
}

macro_rules! with_set_low_word_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_set_low_word in module {}", module_path!());
    };
}

mkfn!{
    with_set_low_word_introspect!();
    # [inline] fn with_set_low_word (f : f64 , lo : u32) -> f64 { let mut tmp = f . to_bits () ; tmp &= 0xffffffff_00000000 ; tmp |= lo as u64 ; f64 :: from_bits (tmp) }
}

macro_rules! combine_words_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function combine_words in module {}", module_path!());
    };
}

mkfn!{
    combine_words_introspect!();
    # [inline] fn combine_words (hi : u32 , lo : u32) -> f64 { f64 :: from_bits (((hi as u64) << 32) | lo as u64) }
}