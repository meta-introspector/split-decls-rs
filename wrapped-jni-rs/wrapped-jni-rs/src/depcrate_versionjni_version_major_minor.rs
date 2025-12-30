// Generated macro for jni_version_major_minor (function)
macro_rules! Depcrate_versionjni_version_major_minor {
() => {
// Module: crate::version
// Provides: {"jni_version_major_minor"}
// Dependencies: {}
# [test] fn jni_version_major_minor () { macro_rules ! check_major_minor { ($ major : expr , $ minor : expr , $ jni_ver : tt , $ jni_sys_ver : tt) => { let v = JNIVersion ::$ jni_ver ; assert_eq ! (v . major () , $ major) ; assert_eq ! (v . minor () , $ minor) ; let v = JNIVersion :: new (jni_sys ::$ jni_sys_ver) ; assert_eq ! (v . major () , $ major) ; assert_eq ! (v . minor () , $ minor) ; } ; } check_major_minor ! (1 , 1 , V1_1 , JNI_VERSION_1_1) ; check_major_minor ! (1 , 2 , V1_2 , JNI_VERSION_1_2) ; check_major_minor ! (1 , 4 , V1_4 , JNI_VERSION_1_4) ; check_major_minor ! (1 , 6 , V1_6 , JNI_VERSION_1_6) ; check_major_minor ! (1 , 8 , V1_8 , JNI_VERSION_1_8) ; check_major_minor ! (9 , 0 , V9 , JNI_VERSION_9) ; check_major_minor ! (10 , 0 , V10 , JNI_VERSION_10) ; check_major_minor ! (19 , 0 , V19 , JNI_VERSION_19) ; check_major_minor ! (20 , 0 , V20 , JNI_VERSION_20) ; check_major_minor ! (21 , 0 , V21 , JNI_VERSION_21) ; }
};
}
