// Generated macro for uncached (function)
macro_rules! Depcrateuncached {
() => {
// Module: crate
// Provides: {"uncached"}
// Dependencies: {}
# [doc = " Returns true if the current environment is found to probably be a CI"] # [doc = " environment or service. If you expect to call this multiple times without"] # [doc = " the environment changing, use [cached]."] pub fn uncached () -> bool { let ci_var = std :: env :: var ("CI") ; ci_var == Ok ("true" . into ()) || ci_var == Ok ("1" . into ()) || ci_var == Ok ("woodpecker" . into ()) || check ("CI_NAME") || check ("GITHUB_ACTION") || check ("GITLAB_CI") || check ("NETLIFY") || check ("TRAVIS") || matches ! (std :: env :: var ("NODE") , Ok (node) if node . ends_with ("//heroku/node/bin/node")) || check ("CODEBUILD_SRC_DIR") || check ("BUILDER_OUTPUT") || check ("GITLAB_DEPLOYMENT") || check ("NOW_GITHUB_DEPLOYMENT") || check ("NOW_BUILDER") || check ("BITBUCKET_DEPLOYMENT") || check ("GERRIT_PROJECT") || check ("SYSTEM_TEAMFOUNDATIONCOLLECTIONURI") || check ("BITRISE_IO") || check ("BUDDY_WORKSPACE_ID") || check ("BUILDKITE") || check ("CIRRUS_CI") || check ("APPVEYOR") || check ("CIRCLECI") || check ("SEMAPHORE") || check ("DRONE") || check ("DSARI") || check ("TDDIUM") || check ("STRIDER") || check ("TASKCLUSTER_ROOT_URL") || check ("JENKINS_URL") || check ("bamboo.buildKey") || check ("GO_PIPELINE_NAME") || check ("HUDSON_URL") || check ("WERCKER") || check ("MAGNUM") || check ("NEVERCODE") || check ("RENDER") || check ("SAIL_CI") || check ("SHIPPABLE") }
};
}
