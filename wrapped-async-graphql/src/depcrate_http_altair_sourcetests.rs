// Generated macro for tests (module)
macro_rules! Depcrate_http_altair_sourcetests {
() => {
// Module: crate::http::altair_source
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use serde_json :: json ; use super :: * ; # [test] fn test_without_options () { let altair_source = AltairSource :: build () . title ("Custom Title") . finish () ; assert_eq ! (altair_source , r#"<!DOCTYPE html>
<html>

  <head>
    <meta charset="utf-8">

    <title>Custom Title</title>

    <base href="https://unpkg.com/altair-static@latest/build/dist/">

    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="icon" type="image/x-icon" href="favicon.ico">
    <link rel="stylesheet" href="styles.css">
  </head>

  <body>
    <script>
      document.addEventListener('DOMContentLoaded', () => {
        AltairGraphQL.init();
      });
    </script>
    <app-root>
      <style>
        .loading-screen {
          /*Prevents the loading screen from showing until CSS is downloaded*/
          display: none;
        }
      </style>
      <div class="loading-screen styled">
        <div class="loading-screen-inner">
          <div class="loading-screen-logo-container">
            <img src="assets/img/logo_350.svg" alt="Altair">
          </div>
          <div class="loading-screen-loading-indicator">
            <span class="loading-indicator-dot"></span>
            <span class="loading-indicator-dot"></span>
            <span class="loading-indicator-dot"></span>
          </div>
        </div>
      </div>
    </app-root>
    <script type="text/javascript" src="runtime.js"></script>
    <script type="text/javascript" src="polyfills.js"></script>
    <script type="text/javascript" src="main.js"></script>
  </body>

</html>"#) } # [test] fn test_with_dynamic () { let altair_source = AltairSource :: build () . options (json ! ({ "endpointURL" : "/" , "subscriptionsEndpoint" : "/ws" , })) . finish () ; assert_eq ! (altair_source , r#"<!DOCTYPE html>
<html>

  <head>
    <meta charset="utf-8">

    <title>Altair</title>

    <base href="https://unpkg.com/altair-static@latest/build/dist/">

    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="icon" type="image/x-icon" href="favicon.ico">
    <link rel="stylesheet" href="styles.css">
  </head>

  <body>
    <script>
      document.addEventListener('DOMContentLoaded', () => {
        AltairGraphQL.init({"endpointURL":"/","subscriptionsEndpoint":"/ws"});
      });
    </script>
    <app-root>
      <style>
        .loading-screen {
          /*Prevents the loading screen from showing until CSS is downloaded*/
          display: none;
        }
      </style>
      <div class="loading-screen styled">
        <div class="loading-screen-inner">
          <div class="loading-screen-logo-container">
            <img src="assets/img/logo_350.svg" alt="Altair">
          </div>
          <div class="loading-screen-loading-indicator">
            <span class="loading-indicator-dot"></span>
            <span class="loading-indicator-dot"></span>
            <span class="loading-indicator-dot"></span>
          </div>
        </div>
      </div>
    </app-root>
    <script type="text/javascript" src="runtime.js"></script>
    <script type="text/javascript" src="polyfills.js"></script>
    <script type="text/javascript" src="main.js"></script>
  </body>

</html>"#) } # [test] fn test_with_static () { let altair_source = AltairSource :: build () . options (AltairConfigOptions { window_options : Some (AltairWindowOptions { endpoint_url : Some ("/" . to_owned ()) , subscriptions_endpoint : Some ("/ws" . to_owned ()) , .. Default :: default () }) , .. Default :: default () }) . finish () ; assert_eq ! (altair_source , r#"<!DOCTYPE html>
<html>

  <head>
    <meta charset="utf-8">

    <title>Altair</title>

    <base href="https://unpkg.com/altair-static@latest/build/dist/">

    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="icon" type="image/x-icon" href="favicon.ico">
    <link rel="stylesheet" href="styles.css">
  </head>

  <body>
    <script>
      document.addEventListener('DOMContentLoaded', () => {
        AltairGraphQL.init({"endpointURL":"/","subscriptionsEndpoint":"/ws"});
      });
    </script>
    <app-root>
      <style>
        .loading-screen {
          /*Prevents the loading screen from showing until CSS is downloaded*/
          display: none;
        }
      </style>
      <div class="loading-screen styled">
        <div class="loading-screen-inner">
          <div class="loading-screen-logo-container">
            <img src="assets/img/logo_350.svg" alt="Altair">
          </div>
          <div class="loading-screen-loading-indicator">
            <span class="loading-indicator-dot"></span>
            <span class="loading-indicator-dot"></span>
            <span class="loading-indicator-dot"></span>
          </div>
        </div>
      </div>
    </app-root>
    <script type="text/javascript" src="runtime.js"></script>
    <script type="text/javascript" src="polyfills.js"></script>
    <script type="text/javascript" src="main.js"></script>
  </body>

</html>"#) } }
};
}
